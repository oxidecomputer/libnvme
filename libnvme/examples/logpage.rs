// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nvme = libnvme::Nvme::new()?;
    let discovery = nvme.controller_discovery()?;

    for controller in discovery.into_iter() {
        let controller = controller?;
        let firmware = controller.get_firmware_log_page()?;
        let slots = firmware.slot_iter().collect::<Vec<_>>();
        println!("{slots:#?}");
    }

    Ok(())
}
