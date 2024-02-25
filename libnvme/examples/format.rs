// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use libnvme::namespace::NamespaceDiscoveryLevel;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nvme = libnvme::Nvme::new()?;
    let discovery = nvme.controller_discovery()?;

    for controller in discovery.into_iter() {
        let controller =
            controller?.write_lock().map_err(|(_controller, e)| e)?;
        let info = controller.get_info()?;
        if info.serial() == "NVME-5-0" {
            let nsdisc = controller
                .namespace_discovery(NamespaceDiscoveryLevel::Active)?;
            let namespaces =
                nsdisc.into_iter().collect::<Result<Vec<_>, _>>()?;
            assert_eq!(namespaces.len(), 1, "single active namespace");

            namespaces.iter().try_for_each(|ns| ns.blkdev_detach())?;

            let lba = info
                .lba_formats()
                .into_iter()
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .find(|lba| lba.meta_size() == 0 && lba.data_size() == 512)
                .ok_or_else(|| {
                    std::io::Error::other("couldn't find expected lba format")
                })?
                .id();

            controller
                .format_request()?
                .set_lbaf(lba)?
                .set_nsid(u32::MAX)?
                .set_ses(0)?
                .execute()?;

            namespaces.iter().try_for_each(|ns| ns.blkdev_attach())?;

            println!(
                "successfully formatted nvme controller: {}",
                info.serial()
            );
        }
    }

    Ok(())
}
