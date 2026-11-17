# {if:just}
# Audit dependencies against the RustSec advisory DB
[group('security')]
audit:
    cargo audit
# {endif:just}
