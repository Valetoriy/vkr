#!/usr/bin/env bash

cargo objcopy --example $(basename $1) -- -O ihex target/out.hex
python "/home/spff/.platformio/packages/tool-mik32-uploader/mik32_upload.py" target/out.hex --openocd-exec /home/spff/.platformio/packages/tool-openocd/bin/openocd --adapter-speed 500 --openocd-scripts /home/spff/.platformio/packages/tool-openocd/openocd/scripts --openocd-target /home/spff/.platformio/packages/tool-mik32-uploader/openocd-scripts/target/mik32.cfg --run-openocd --mcu-type MIK32V2 --openocd-interface /home/spff/.platformio/packages/tool-mik32-uploader/openocd-scripts/interface/ftdi/mikron-link.cfg
