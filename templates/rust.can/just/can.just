# {if:just}
# ── CAN (virtual bus for dev) ─────────────────────────────────

vcan := "vcan0"

# Create and bring up the virtual CAN bus (needs sudo)
[group("can")]
vcan-up:
    sudo modprobe vcan
    ip link show {{ vcan }} >/dev/null 2>&1 || sudo ip link add dev {{ vcan }} type vcan
    sudo ip link set up {{ vcan }}
    @echo "✅ {{ vcan }} is up"

# Tear down the virtual CAN bus
[group("can")]
vcan-down:
    sudo ip link delete {{ vcan }}

# Watch all frames on the bus
[group("can")]
can-dump:
    candump {{ vcan }}

# Send one frame: just can-send 501 DEADBEEF
[group("can")]
can-send ID DATA="":
    cansend {{ vcan }} {{ ID }}#{{ DATA }}

# Generate continuous frames: just can-gen 502 [gap-ms]
[group("can")]
can-gen ID GAP="100":
    cangen {{ vcan }} -I {{ ID }} -g {{ GAP }} -v
# {endif:just}
