#[cfg(test)]
mod tests {
    use vjoy_sys::AXES_HID_USAGE;

    #[test]
    fn open_device() {
        unsafe {
            //Success of this test can be observed via the vJoyMonitor and vJoyList executables bundled with vJoy.

            let vjoy = vjoy_sys::vJoyInterface::new("C:/Program Files/vJoy/x64/vJoyInterface.dll")
                .unwrap();
            assert!(vjoy.vJoyEnabled() == 1, "vJoy is not installed");
            assert!(vjoy.isVJDExists(1) == 1, "device 1 does not exist");
            assert!(
                vjoy.AcquireVJD(1) == 1,
                "device 1 is busy and could not be acquired"
            );

            println!("Setting button 1 of device 1");
            assert!(
                vjoy.SetBtn(1, 1, 1) == 1,
                "button 1 of device 1 could not be set"
            );
            std::thread::sleep(std::time::Duration::from_millis(1000));

            println!("Resetting button 1 of device 1");
            assert!(
                vjoy.SetBtn(0, 1, 1) == 1,
                "button 1 of device 1 could not be reset"
            );
            std::thread::sleep(std::time::Duration::from_millis(1000));


            println!("Setting axis 1 of device 1");
            assert!(
                vjoy.SetAxis(32_767_i32, 1, AXES_HID_USAGE[0]) == 1,
                "axis 1 of device 1 could not be set"
            );
            std::thread::sleep(std::time::Duration::from_millis(1000));

            println!("Resetting axis 1 of device 1");
            assert!(
                vjoy.SetAxis(0, 1, AXES_HID_USAGE[0]) == 1,
                "axis 1 of device 1 could not be reset"
            );
            std::thread::sleep(std::time::Duration::from_millis(1000));

            let num_cont_hat = vjoy.GetVJDContPovNumber(1);
            let num_disc_hat = vjoy.GetVJDDiscPovNumber(1);

            if num_disc_hat >= 1{
                println!("Setting discrete hat 1 of device 1 NORTH");
                assert!(
                    vjoy.SetDiscPov(0, 1, 1) == 1,
                    "discrete hat 1 of device 1 could not be set"
                );
                std::thread::sleep(std::time::Duration::from_millis(1000));
    
                println!("Setting discrete hat 1 of device 1 EAST");
                assert!(
                    vjoy.SetDiscPov(1, 1, 1) == 1,
                    "discrete hat 1 of device 1 could not be set"
                );
                std::thread::sleep(std::time::Duration::from_millis(1000));
    
                println!("Setting discrete hat 1 of device 1 SOUTH");
                assert!(
                    vjoy.SetDiscPov(2, 1, 1) == 1,
                    "discrete hat 1 of device 1 could not be set"
                );
                std::thread::sleep(std::time::Duration::from_millis(1000));
    
                println!("Setting discrete hat 1 of device 1 WEST");
                assert!(
                    vjoy.SetDiscPov(3, 1, 1) == 1,
                    "discrete hat 1 of device 1 could not be set"
                );
                std::thread::sleep(std::time::Duration::from_millis(1000));
    
                println!("Resetting discrete hat 1 of device 1");
                assert!(
                    vjoy.SetDiscPov(-1, 1, 1) == 1,
                    "discrete hat 1 of device 1 could not be reset"
                );
                std::thread::sleep(std::time::Duration::from_millis(1000));
            }

            if num_cont_hat >= 1{
                println!("Setting continuous hat 1 of device 1 45°");
                assert!(
                    vjoy.SetContPov(45 * 100, 1, 1) == 1,
                    "continuous hat 1 of device 1 could not be set"
                );
                std::thread::sleep(std::time::Duration::from_millis(1000));
    
                println!("Setting continuous hat 1 of device 1 135°");
                assert!(
                    vjoy.SetContPov(135 * 100, 1, 1) == 1,
                    "continuous hat 1 of device 1 could not be set"
                );
                std::thread::sleep(std::time::Duration::from_millis(1000));
    
                println!("Setting continuous hat 1 of device 1 225°");
                assert!(
                    vjoy.SetContPov(225 * 100, 1, 1) == 1,
                    "continuous hat 1 of device 1 could not be set"
                );
                std::thread::sleep(std::time::Duration::from_millis(1000));
    
                println!("Setting continuous hat 1 of device 1 315°");
                assert!(
                    vjoy.SetContPov(315 * 100, 1, 1) == 1,
                    "continuous hat 1 of device 1 could not be set"
                );
                std::thread::sleep(std::time::Duration::from_millis(1000));
    
                println!("Resetting continuous hat 1 of device 1");
                assert!(
                    vjoy.SetContPov(u32::MAX, 1, 1) == 1,
                    "continuous hat 1 of device 1 could not be reset"
                );
                std::thread::sleep(std::time::Duration::from_millis(1000));
            }

            vjoy.RelinquishVJD(1);
        }
    }
}
