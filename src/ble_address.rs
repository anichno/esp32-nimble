use core::hash::{Hash, Hasher};
use esp_idf_sys::*;

/// Bluetooth Device address type
pub enum BLEAddressType {
  Public = BLE_ADDR_PUBLIC as _,
  Random = BLE_ADDR_RANDOM as _,
  PublicID = BLE_ADDR_PUBLIC_ID as _,
  RandomID = BLE_ADDR_RANDOM_ID as _,
}

#[derive(Copy, Clone, Default)]
pub struct BLEAddress {
  pub(crate) value: esp_idf_sys::ble_addr_t,
}

impl PartialEq for BLEAddress {
  fn eq(&self, other: &Self) -> bool {
    self.value.val == other.value.val && self.value.type_ == other.value.type_
  }
}

impl Eq for BLEAddress {}

impl Hash for BLEAddress {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.value.val.hash(state);
    self.value.type_.hash(state);
  }
}

impl BLEAddress {
  pub fn new(val: [u8; 6], addr_type: BLEAddressType) -> Self {
    let mut ret = Self {
      value: esp_idf_sys::ble_addr_t {
        val,
        type_: addr_type as _,
      },
    };
    ret.value.val.reverse();
    ret
  }

  pub fn new_from_addr(val: [u8; 6]) -> Self {
    let mut ret = Self {
      value: esp_idf_sys::ble_addr_t {
        val,
        type_: BLEAddressType::Public as _,
      },
    };
    ret.value.val.reverse();
    ret
  }
}

impl From<esp_idf_sys::ble_addr_t> for BLEAddress {
  fn from(value: esp_idf_sys::ble_addr_t) -> Self {
    Self { value }
  }
}

impl core::fmt::Display for BLEAddress {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(
      f,
      "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
      self.value.val[5],
      self.value.val[4],
      self.value.val[3],
      self.value.val[2],
      self.value.val[1],
      self.value.val[0]
    )
  }
}

impl core::fmt::Debug for BLEAddress {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "{self}")
  }
}
