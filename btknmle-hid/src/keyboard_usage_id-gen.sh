#!/bin/bash

base=$(dirname $0)
base=$(cd $base && pwd)
source=$base/keyboard_usage_id.txt
#target=$base/keyboard_usage_id.rs

echo '#![allow(non_camel_case_types)]'
echo 'use btknmle_input::KeyCodes;'
echo ''
grep -ve'^#' $source | sed -re's/^(^[^[:space:]]+)[[:space:]]+([^[:space:]]+)/const KEY_\1:u8 = 0x\2;/g'

echo ''
echo '#[derive(Debug)]'
echo 'pub enum KeyboardUsageId {'
grep -ve'^#' $source | sed -re's/^(^[^[:space:]]+)[[:space:]]+([^[:space:]]+)/\tKEY_\1,/g'
echo '  Unknown(u8),'
echo '}'

echo ''
echo 'impl KeyboardUsageId {'
echo 'pub fn from_keycodes(v: KeyCodes) -> Option<Self> {'
echo 'Some(match v {'
grep -ve'^#' $source | sed -re's/^(^[^[:space:]]+)[[:space:]]+([^[:space:]]+)/\tKeyCodes::KEY_\1 => Self::KEY_\1,/g'
echo '_ => return None,'
echo '})'
echo '}'
echo '}'

echo ''
echo 'impl From<KeyboardUsageId> for u8 {'
echo 'fn from(v: KeyboardUsageId) -> Self {'
echo 'match v {'
grep -ve'^#' $source | sed -re's/^(^[^[:space:]]+)[[:space:]]+([^[:space:]]+)/\tKeyboardUsageId::KEY_\1 => KEY_\1,/g'
echo 'KeyboardUsageId::Unknown(v) => v,'
echo '}'
echo '}'
echo '}'
