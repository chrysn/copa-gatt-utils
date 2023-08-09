<!--
SPDX-FileCopyrightText: Copyright 2022 EDF (Électricité de France S.A.)
SPDX-License-Identifier: BSD-3-Clause
-->
# coap-gatt-utils

Implementation utilities for CoAP-over-GATT ([draft-amsuess-core-coap-over-gatt-02])
<!-- When updating this line, don't forget to update the description in Cargo.toml -->

Right now, this contains the message format parsing and serialization, largely fulled by
[coap_message_utils], given that most of the message format is just a rehash of the universal
CoAP option-extension-data-ff-payload scheme.

[draft-amsuess-core-coap-over-gatt-02]: https://datatracker.ietf.org/doc/id/draft-amsuess-core-coap-over-gatt-02.html

License
-------

This project and all files contained in it is published under the
BSD-3-Clause license as defined in [`LICENSES/BSD-3-Clause.txt`](LICENSES/BSD-3-Clause.txt).

Copyright: 2022 EDF (Électricité de France S.A.)

Author: Christian Amsüss
