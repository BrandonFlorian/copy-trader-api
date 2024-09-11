-- For the users table
CREATE POLICY "Users can access their own data or service role can insert" ON users
  USING (wallet_address = auth.uid()::text OR auth.role() = 'service_role')
  WITH CHECK (wallet_address = auth.uid()::text OR auth.role() = 'service_role');

ALTER TABLE users ENABLE ROW LEVEL SECURITY;

-- For the tracked_wallets table
CREATE POLICY "Users can only access their own tracked wallets" ON tracked_wallets
  USING (user_id = auth.uid()::text OR auth.role() = 'service_role')
  WITH CHECK (user_id = auth.uid()::text OR auth.role() = 'service_role');

ALTER TABLE tracked_wallets ENABLE ROW LEVEL SECURITY;

-- For the copy_trade_settings table
CREATE POLICY "Users can only access their own copy trade settings" ON copy_trade_settings
  USING (user_id = auth.uid()::text OR auth.role() = 'service_role')
  WITH CHECK (user_id = auth.uid()::text OR auth.role() = 'service_role');

ALTER TABLE copy_trade_settings ENABLE ROW LEVEL SECURITY;

-- For the transactions table
CREATE POLICY "Users can only view their own transactions" ON transactions
  USING (user_id = auth.uid()::text);

ALTER TABLE transactions ENABLE ROW LEVEL SECURITY;