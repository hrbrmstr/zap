# zap

Weatherflow Tempest weather station lightning strike logger.

Tempest weather stations (via the base station) send JSON records over UDP broadcast.

The `evt_strike` one logs lightning strikes:

```json
{"serial_number":"ST-00055227","type":"evt_strike","hub_sn":"HB-00069665","evt":[1645548480,63,17825791]}`
```

The `evt` array is:

- index 0: Time Epoch (in seconds)
- index 1: Distance	(km)
- index 2: Energy	(?)

Redirect `stdout` to something useful to log. Ctrl-c to exit.