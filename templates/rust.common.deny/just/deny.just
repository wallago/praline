# {if:just}
# Check licenses, banned crates, sources and advisories
[group('security')]
deny:
    cargo deny check
# {endif:just}
