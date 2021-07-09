// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    service::HapService,
    characteristic::{
        HapCharacteristic,
		current_slat_state::CurrentSlatStateCharacteristic,
		slat_type::SlatTypeCharacteristic,
		name::NameCharacteristic,
		swing_mode::SwingModeCharacteristic,
		current_tilt_angle::CurrentTiltAngleCharacteristic,
		target_tilt_angle::TargetTiltAngleCharacteristic,
	},
    HapType,
};

/// Slats Service.
#[derive(Debug, Default)]
pub struct SlatsService {
    /// Instance ID of the Slats Service.
    id: u64,
    /// `HapType` of the Slats Service.
    hap_type: HapType,
    /// When set to true, this service is not visible to user.
    hidden: bool,
    /// When set to true, this is the primary service on the accessory.
    primary: bool,
    /// An array of numbers containing the instance IDs of the services that this service links to.
    linked_services: Vec<u64>,

	/// Current Slat State Characteristic (required).
	pub current_slat_state: CurrentSlatStateCharacteristic,
	/// Slat Type Characteristic (required).
	pub slat_type: SlatTypeCharacteristic,

	/// Name Characteristic (optional).
	pub name: Option<NameCharacteristic>,
	/// Swing Mode Characteristic (optional).
	pub swing_mode: Option<SwingModeCharacteristic>,
	/// Current Tilt Angle Characteristic (optional).
	pub current_tilt_angle: Option<CurrentTiltAngleCharacteristic>,
	/// Target Tilt Angle Characteristic (optional).
	pub target_tilt_angle: Option<TargetTiltAngleCharacteristic>,
}

impl SlatsService {
    /// Creates a new Slats Service.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        Self {
            id,
            hap_type: HapType::Slats,
			current_slat_state: CurrentSlatStateCharacteristic::new(id + 1 + 0, accessory_id),
			slat_type: SlatTypeCharacteristic::new(id + 1 + 1, accessory_id),
			name: Some(NameCharacteristic::new(id + 1 + 0 + 2, accessory_id)),
			swing_mode: Some(SwingModeCharacteristic::new(id + 1 + 1 + 2, accessory_id)),
			current_tilt_angle: Some(CurrentTiltAngleCharacteristic::new(id + 1 + 2 + 2, accessory_id)),
			target_tilt_angle: Some(TargetTiltAngleCharacteristic::new(id + 1 + 3 + 2, accessory_id)),
			..Default::default()
        }
    }
}

impl HapService for SlatsService {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_type(&self) -> HapType {
        self.hap_type
    }

    fn get_hidden(&self) -> bool {
        self.hidden
    }

    fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    fn get_primary(&self) -> bool {
        self.primary
    }

    fn set_primary(&mut self, primary: bool) {
        self.primary = primary;
    }

    fn get_linked_services(&self) -> Vec<u64> {
        self.linked_services.clone()
    }

    fn set_linked_services(&mut self, linked_services: Vec<u64>) {
        self.linked_services = linked_services;
    }

    fn get_characteristic(&self, hap_type: HapType) -> Option<&dyn HapCharacteristic> {
        for characteristic in self.get_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_mut_characteristic(&mut self, hap_type: HapType) -> Option<&mut dyn HapCharacteristic> {
        for characteristic in self.get_mut_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        #[allow(unused_mut)]
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![
			&self.current_slat_state,
			&self.slat_type,
		];
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &self.swing_mode {
		    characteristics.push(c);
		}
		if let Some(c) = &self.current_tilt_angle {
		    characteristics.push(c);
		}
		if let Some(c) = &self.target_tilt_angle {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        #[allow(unused_mut)]
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.current_slat_state,
			&mut self.slat_type,
		];
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.swing_mode {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.current_tilt_angle {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.target_tilt_angle {
		    characteristics.push(c);
		}
		characteristics
    }
}

impl Serialize for SlatsService {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapService", 5)?;
        state.serialize_field("iid", &self.get_id())?;
        state.serialize_field("type", &self.get_type())?;
        state.serialize_field("hidden", &self.get_hidden())?;
        state.serialize_field("primary", &self.get_primary())?;
        state.serialize_field("characteristics", &self.get_characteristics())?;
        // linked services left out for now
        state.end()
    }
}