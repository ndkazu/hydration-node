#![cfg(test)]

use crate::polkadot_test_net::*;
use common_runtime::BlockNumber;
use frame_support::traits::OnFinalize;
use frame_support::traits::OnInitialize;
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;
use hydradx_runtime::{Balances, CircuitBreaker, Omnipool, OmnipoolCollectionId, Tokens, Uniques};
use orml_traits::MultiCurrency;
use primitives::constants::chain::CORE_ASSET_ID;
use primitives::Balance;
use sp_runtime::FixedU128;
use sp_runtime::Permill;
use xcm_emulator::TestExt;
use sp_runtime::traits::Zero;

#[test]
fn sell_in_omnipool_should_work_when_max_trade_limit_per_block_not_exceeded() {
	Hydra::execute_with(|| {
		//Arrange
		init_omnipool();

		let dai_balance_in_omnipool = Tokens::free_balance(DAI, &Omnipool::protocol_account());
		let trade_volume_limit = CircuitBreaker::trade_volume_limit_per_asset(DAI);
		let sell_amount = CircuitBreaker::calculate_limit(dai_balance_in_omnipool, trade_volume_limit).unwrap();

		assert_ok!(Tokens::set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			DAI,
			sell_amount,
			0,
		));

		let min_limit = 0;

		hydradx_run_to_block(3);

		//Act and assert
		assert_ok!(Omnipool::sell(
			hydradx_runtime::Origin::signed(ALICE.into()),
			DAI,
			CORE_ASSET_ID,
			sell_amount,
			min_limit
		));
	});
}

#[test]
fn sell_in_omnipool_should_fail_when_max_trade_limit_per_block_exceeded() {
	Hydra::execute_with(|| {
		//Arrange
		init_omnipool();

		let dai_balance_in_omnipool = Tokens::free_balance(DAI, &Omnipool::protocol_account());
		let trade_volume_limit = CircuitBreaker::trade_volume_limit_per_asset(DAI);
		let sell_amount = CircuitBreaker::calculate_limit(dai_balance_in_omnipool, trade_volume_limit)
			.unwrap()
			.checked_add(1)
			.unwrap();

		assert_ok!(Tokens::set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			DAI,
			sell_amount,
			0,
		));

		let min_limit = 0;

		hydradx_run_to_block(3);

		//Act and assert
		assert_noop!(
			Omnipool::sell(
				hydradx_runtime::Origin::signed(ALICE.into()),
				DAI,
				CORE_ASSET_ID,
				sell_amount,
				min_limit
			),
			pallet_circuit_breaker::Error::<hydradx_runtime::Runtime>::MaxTradeVolumePerBlockReached
		);
	});
}

#[test]
fn sell_lrna_in_omnipool_should_fail_when_min_trade_limit_per_block_exceeded() {
	Hydra::execute_with(|| {
		//Arrange
		init_omnipool();

		let lrna_balance_in_omnipool = Tokens::free_balance(LRNA, &Omnipool::protocol_account());
		let trade_volume_limit = CircuitBreaker::trade_volume_limit_per_asset(LRNA);
		let sell_amount = CircuitBreaker::calculate_limit(lrna_balance_in_omnipool, trade_volume_limit)
			.unwrap()
			.checked_add(1)
			.unwrap();

		assert_ok!(Tokens::set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			LRNA,
			sell_amount,
			0,
		));

		let min_limit = 0;

		hydradx_run_to_block(3);

		//Act and assert
		assert_noop!(
			Omnipool::sell(
				hydradx_runtime::Origin::signed(ALICE.into()),
				LRNA,
				CORE_ASSET_ID,
				sell_amount,
				min_limit
			),
			pallet_circuit_breaker::Error::<hydradx_runtime::Runtime>::MinTradeVolumePerBlockReached
		);
	});
}

#[test]
fn buy_asset_for_lrna_should_fail_when_min_trade_limit_per_block_exceeded() {
	Hydra::execute_with(|| {
		//Arrange
		init_omnipool();

		assert_ok!(Tokens::set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			LRNA,
			100000000000 * UNITS,
			0,
		));

		hydradx_run_to_block(3);

		//Act and assert
		assert_noop!(
			Omnipool::buy(
				hydradx_runtime::Origin::signed(ALICE.into()),
				CORE_ASSET_ID,
				LRNA,
				200000 * UNITS,
				Balance::MAX
			),
			pallet_circuit_breaker::Error::<hydradx_runtime::Runtime>::MinTradeVolumePerBlockReached
		);
	});
}

#[test]
fn buy_in_omnipool_should_work_when_max_trade_limit_per_block_not_exceeded() {
	Hydra::execute_with(|| {
		//Arrange
		init_omnipool();

		assert_ok!(Tokens::set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			DAI,
			10000000 * UNITS,
			0,
		));

		hydradx_run_to_block(3);

		assert_ok!(Omnipool::buy(
			hydradx_runtime::Origin::signed(ALICE.into()),
			CORE_ASSET_ID,
			DAI,
			UNITS,
			Balance::MAX
		));
	});
}

#[test]
fn buy_in_omnipool_should_fail_when_max_trade_limit_per_block_exceeded() {
	Hydra::execute_with(|| {
		//Arrange
		init_omnipool();

		assert_ok!(Tokens::set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			DAI,
			100000000000 * UNITS,
			0,
		));

		hydradx_run_to_block(3);

		//Act and assert
		assert_noop!(
			Omnipool::buy(
				hydradx_runtime::Origin::signed(ALICE.into()),
				CORE_ASSET_ID,
				DAI,
				100000 * UNITS,
				Balance::MAX
			),
			pallet_circuit_breaker::Error::<hydradx_runtime::Runtime>::MaxTradeVolumePerBlockReached
		);
	});
}

#[test]
fn add_liquidity_to_omnipool_should_work_when_liquidity_limit_per_block_not_exceeded() {
	Hydra::execute_with(|| {
		//Arrange
		hydradx_run_to_block(2);

		init_omnipool();

		let hdx_balance_in_omnipool = Balances::free_balance(&Omnipool::protocol_account());
		let liquidity_limit = CircuitBreaker::add_liquidity_limit_per_asset(CORE_ASSET_ID).unwrap();
		let added_liquidity = CircuitBreaker::calculate_limit(hdx_balance_in_omnipool, liquidity_limit).unwrap();

		assert_ok!(Balances::set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			added_liquidity,
			0,
		));

		hydradx_run_to_block(3);

		//Act and assert
		assert_ok!(Omnipool::add_liquidity(
			hydradx_runtime::Origin::signed(ALICE.into()),
			CORE_ASSET_ID,
			added_liquidity,
		));
	});
}

#[test]
fn add_liquidity_to_omnipool_should_fail_when_liquidity_limit_per_block_exceeded() {
	Hydra::execute_with(|| {
		//Arrange
		init_omnipool();

		let hdx_balance_in_omnipool = Balances::free_balance(&Omnipool::protocol_account());
		let liquidity_limit = CircuitBreaker::add_liquidity_limit_per_asset(CORE_ASSET_ID).unwrap();
		let added_liquidity = CircuitBreaker::calculate_limit(hdx_balance_in_omnipool, liquidity_limit)
			.unwrap()
			.checked_add(1)
			.unwrap();

		assert_ok!(Balances::set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			added_liquidity,
			0,
		));

		hydradx_run_to_block(3);

		//Act and assert
		assert_noop!(
			Omnipool::add_liquidity(
				hydradx_runtime::Origin::signed(ALICE.into()),
				CORE_ASSET_ID,
				added_liquidity,
			),
			pallet_circuit_breaker::Error::<hydradx_runtime::Runtime>::MaxLiquidityLimitPerBlockReached
		);
	});
}

#[test]
fn add_liquidity_to_omnipool_should_not_fail_when_liquidity_limit_per_block_exceeded_but_called_by_whitelisted() {
	Hydra::execute_with(|| {
		//Arrange
		init_omnipool();

		let hdx_balance_in_omnipool = Balances::free_balance(&Omnipool::protocol_account());
		let liquidity_limit = CircuitBreaker::add_liquidity_limit_per_asset(CORE_ASSET_ID).unwrap();
		let added_liquidity = CircuitBreaker::calculate_limit(hdx_balance_in_omnipool, liquidity_limit)
			.unwrap()
			.checked_add(1)
			.unwrap();

		assert_ok!(Balances::set_balance(
			RawOrigin::Root.into(),
			Treasury::account_id(),
			added_liquidity,
			0,
		));

		hydradx_run_to_block(3);

		//Act and assert
		assert_ok!(Omnipool::add_liquidity(
			hydradx_runtime::Origin::signed(Treasury::account_id()),
			CORE_ASSET_ID,
			added_liquidity,
		));
	});
}

#[test]
fn remove_liquidity_to_omnipool_should_work_when_liquidity_limit_per_block_not_exceeded() {
	Hydra::execute_with(|| {
		//Arrange
		init_omnipool();

		let hdx_balance_in_omnipool = Balances::free_balance(&Omnipool::protocol_account());
		let liquidity_limit = CircuitBreaker::add_liquidity_limit_per_asset(CORE_ASSET_ID).unwrap();
		let added_liquidity = CircuitBreaker::calculate_limit(hdx_balance_in_omnipool, liquidity_limit).unwrap();

		assert_ok!(Balances::set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			added_liquidity,
			0,
		));

		hydradx_run_to_block(3);

		//Act and assert
		let position_id = Omnipool::next_position_id();
		assert_ok!(Omnipool::add_liquidity(
			hydradx_runtime::Origin::signed(ALICE.into()),
			CORE_ASSET_ID,
			added_liquidity,
		));

		assert_ok!(Omnipool::remove_liquidity(
			hydradx_runtime::Origin::signed(ALICE.into()),
			position_id,
			added_liquidity,
		));
	});
}

#[test]
fn remove_liquidity_from_omnipool_should_fail_when_large_legacy_position_removed() {
	Hydra::execute_with(|| {
		//Arrange
		init_omnipool();
		let hdx_balance_in_omnipool = Balances::free_balance(&Omnipool::protocol_account());
		let liquidity_limit = CircuitBreaker::add_liquidity_limit_per_asset(CORE_ASSET_ID).unwrap();
		let max_removed_liquidity = CircuitBreaker::calculate_limit(hdx_balance_in_omnipool, liquidity_limit).unwrap();
		let bag = max_removed_liquidity * 2;
		assert_ok!(Balances::set_balance(
			RawOrigin::Root.into(),
			Treasury::account_id(),
			bag,
			0,
		));

		hydradx_run_to_block(3);

		let position = Omnipool::next_position_id();
		assert_ok!(Omnipool::add_liquidity(
			hydradx_runtime::Origin::signed(Treasury::account_id()),
			CORE_ASSET_ID,
			bag,
		));
		assert_ok!(Uniques::transfer(
			hydradx_runtime::Origin::signed(Treasury::account_id()),
			OmnipoolCollectionId::get(),
			position,
			ALICE.into(),
		));


		//Act and Assert
		assert_noop!(
			Omnipool::remove_liquidity(hydradx_runtime::Origin::signed(ALICE.into()), position, bag,),
			pallet_circuit_breaker::Error::<hydradx_runtime::Runtime>::MaxLiquidityLimitPerBlockReached
		);
	});
}

#[test]
fn remove_liquidity_from_omnipool_should_succeed_when_legacy_position_withdrawn_gradually() {
	Hydra::execute_with(|| {
		//Arrange
		init_omnipool();
		let hdx_balance_in_omnipool = Balances::free_balance(&Omnipool::protocol_account());
		let liquidity_limit = CircuitBreaker::add_liquidity_limit_per_asset(CORE_ASSET_ID).unwrap();
		let max_removed_liquidity = CircuitBreaker::calculate_limit(hdx_balance_in_omnipool, liquidity_limit).unwrap();
		let bag = max_removed_liquidity * 2;
		assert_ok!(Balances::set_balance(
			RawOrigin::Root.into(),
			Treasury::account_id(),
			bag,
			0,
		));

		hydradx_run_to_block(3);

		let position = Omnipool::next_position_id();
		assert_ok!(Omnipool::add_liquidity(
			hydradx_runtime::Origin::signed(Treasury::account_id()),
			CORE_ASSET_ID,
			bag,
		));
		assert_ok!(Uniques::transfer(
			hydradx_runtime::Origin::signed(Treasury::account_id()),
			OmnipoolCollectionId::get(),
			position,
			ALICE.into(),
		));

		//Act and Assert
		assert_ok!(Omnipool::remove_liquidity(
			hydradx_runtime::Origin::signed(ALICE.into()),
			position,
			bag / 2,
		));
		hydradx_run_to_block(4);
		assert_ok!(Omnipool::remove_liquidity(
			hydradx_runtime::Origin::signed(ALICE.into()),
			position,
			bag / 2,
		));
	});
}

#[test]
fn remove_liquidity_to_omnipool_should_fail_when_liquidity_limit_per_block_exceeded() {
	Hydra::execute_with(|| {
		//Arrange
		init_omnipool();

		let hdx_balance_in_omnipool = Balances::free_balance(&Omnipool::protocol_account());
		let liquidity_limit = CircuitBreaker::add_liquidity_limit_per_asset(CORE_ASSET_ID).unwrap();
		let added_liquidity = CircuitBreaker::calculate_limit(hdx_balance_in_omnipool, liquidity_limit).unwrap();

		hydradx_run_to_block(2);

		assert_ok!(Balances::set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			added_liquidity * 10,
			0,
		));

		let position_id_1 = Omnipool::next_position_id();
		assert_ok!(Omnipool::add_liquidity(
			hydradx_runtime::Origin::signed(ALICE.into()),
			CORE_ASSET_ID,
			added_liquidity,
		));

		hydradx_run_to_block(3);

		let position_id_2 = Omnipool::next_position_id();
		assert_ok!(Omnipool::add_liquidity(
			hydradx_runtime::Origin::signed(ALICE.into()),
			CORE_ASSET_ID,
			added_liquidity,
		));

		hydradx_run_to_block(4);

		assert_ok!(Omnipool::remove_liquidity(
			hydradx_runtime::Origin::signed(ALICE.into()),
			position_id_2,
			added_liquidity,
		));

		//Act and Assert
		assert_noop!(
			Omnipool::remove_liquidity(
				hydradx_runtime::Origin::signed(ALICE.into()),
				position_id_1,
				added_liquidity,
			),
			pallet_circuit_breaker::Error::<hydradx_runtime::Runtime>::MaxLiquidityLimitPerBlockReached
		);
	});
}

#[test]
fn remove_liquidity_to_omnipool_should_not_fail_when_liquidity_limit_per_block_exceeded_by_whitelisted_account() {
	Hydra::execute_with(|| {
		//Arrange
		init_omnipool();

		let hdx_balance_in_omnipool = Balances::free_balance(&Omnipool::protocol_account());
		let liquidity_limit = CircuitBreaker::add_liquidity_limit_per_asset(CORE_ASSET_ID).unwrap();
		let added_liquidity = CircuitBreaker::calculate_limit(hdx_balance_in_omnipool, liquidity_limit).unwrap();

		assert_ok!(Balances::set_balance(
			RawOrigin::Root.into(),
			Treasury::account_id(),
			added_liquidity * 10,
			0,
		));

		hydradx_run_to_block(3);

		let position_id_1 = Omnipool::next_position_id();
		assert_ok!(Omnipool::add_liquidity(
			hydradx_runtime::Origin::signed(Treasury::account_id()),
			CORE_ASSET_ID,
			added_liquidity,
		));

		hydradx_run_to_block(2);

		let position_id_2 = Omnipool::next_position_id();
		assert_ok!(Omnipool::add_liquidity(
			hydradx_runtime::Origin::signed(Treasury::account_id()),
			CORE_ASSET_ID,
			added_liquidity,
		));

		hydradx_run_to_block(3);

		assert_ok!(Omnipool::remove_liquidity(
			hydradx_runtime::Origin::signed(Treasury::account_id()),
			position_id_2,
			added_liquidity,
		));

		//Act and Assert
		assert_ok!(Omnipool::remove_liquidity(
			hydradx_runtime::Origin::signed(Treasury::account_id()),
			position_id_1,
			added_liquidity,
		));
	});
}

pub fn hydradx_run_to_block(to: BlockNumber) {
	while hydradx_runtime::System::block_number() < to {
		let b = hydradx_runtime::System::block_number();

		hydradx_runtime::System::on_finalize(b);
		hydradx_runtime::MultiTransactionPayment::on_finalize(b);
		hydradx_runtime::EmaOracle::on_finalize(b);
		hydradx_runtime::CircuitBreaker::on_finalize(b);

		hydradx_runtime::System::on_initialize(b + 1);
		hydradx_runtime::MultiTransactionPayment::on_initialize(b + 1);
		hydradx_runtime::EmaOracle::on_initialize(b + 1);
		hydradx_runtime::CircuitBreaker::on_initialize(b + 1);

		hydradx_runtime::System::set_block_number(b + 1);
	}
}

fn init_omnipool() {
	assert_ok!(hydradx_runtime::Omnipool::set_tvl_cap(
		hydradx_runtime::Origin::root(),
		222_222_000_000_000_000_000_000,
	));

	assert_ok!(Omnipool::initialize_pool(
		RawOrigin::Root.into(),
		FixedU128::from_float(0.00001), // adjust the amount of LRNA to roughly match the amount of LRNA that belongs to HDX. This way we can avoid MaxOutRatioExceeded error.
		FixedU128::from(1),
		Permill::from_percent(100),
		Permill::from_percent(100)
	));

	do_trading_activity_to_populate_oracle();

}




fn do_trading_activity_to_populate_oracle() {
	assert_ok!(Tokens::set_balance(
			RawOrigin::Root.into(),
			BOB.into(),
			DAI,
			UNITS,
			0,
		));

	assert_ok!(Omnipool::sell(
			hydradx_runtime::Origin::signed(BOB.into()),
			DAI,
			CORE_ASSET_ID,
			UNITS,
			Balance::zero()
		));
}