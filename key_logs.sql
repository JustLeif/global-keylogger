--- MySQL Table Creation Script for key_logs
--- MySQL Commands:

--- 1. mysql -u root -p (type password)
--- 2. CREATE DATABASE keylogger;
--- 3. USE keylogger;
--- 4. source <path_to_file>/key_logs.sql
CREATE TABLE key_logs (
	session_id VARCHAR(16),
	key_press VARCHAR(15),
	timestamp_millis BIGINT
);