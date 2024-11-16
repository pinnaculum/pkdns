# Logging

By default pkdns stays silent. Use `--verbose` to make pkdns log all queries.

## Advanced

The log output can be adjusted with the environment variable `RUST_LOG`. This is either done by setting the log level directly (`RUST_LOG=debug`) or by setting the log level for specific modules.

Examples:

- `RUST_LOG=pkdns=trace` will make pkdns very chatty.
- `RUST_LOG=any_dns=trace` will show everything related to ICANN forwarding.
- `RUST_LOG=mainline=debug` will display mainline DHT logs.

These can also be combined: `RUST_LOG=pkdns=trace,any_dns=trace`.

### Interesting Logs

- `RUST_LOG=pkdns=trace,any_dns=trace` Investigate pkdns/anydns.
- `RUST_LOG=pkdns=debug,any_dns=debug,pkarr=debug,mainline=debug` Investigate the mainline DHT.
