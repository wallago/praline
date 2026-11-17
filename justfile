import 'just/nix.just'
import 'just/common.just'
import 'just/udeps.just'
import 'just/deny.just'
import 'just/audit.just'
import 'just/codecov.just'
import 'just/editorconfig.just'
import 'just/typos.just'
import 'just/lychee.just'
import 'just/git.just'
import 'just/github.just'
import 'just/templates.just'
import 'just/release.just'

# List all commands
default:
    @just --list

# Run every check before pushing
[group('ci')]
ci:
    @just nix-lint
    @just fmt-check
    @just toml-check
    @just lint
    @just test
    @just doc
    @just udeps
    @just deny
    @just audit
    @just editorconfig
    @just typos
    @just links
    @just workflows
