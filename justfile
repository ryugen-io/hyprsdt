# hyprdt justfile

default:
    @just --list

# === Install ===

install:
    @./install.sh

# === Build ===

build *args:
    @~/code/.dev/scripts/shared/build/build.sh --release {{args}}

build-debug *args:
    @~/code/.dev/scripts/shared/build/build.sh {{args}}

clean *args:
    @~/code/.dev/scripts/shared/build/clean.sh {{args}}

size:
    @~/code/.dev/scripts/shared/build/size.sh

bloat *args:
    @~/code/.dev/scripts/shared/build/bloat.sh {{args}}

# === Code Quality ===

fmt *args:
    @~/code/.dev/scripts/shared/code/fmt.sh {{args}}

lint *args:
    @~/code/.dev/scripts/shared/code/lint.sh {{args}}

todo:
    @~/code/.dev/scripts/shared/code/todo.sh

# === Dependencies ===

audit:
    @~/code/.dev/scripts/shared/deps/audit.sh

outdated:
    @~/code/.dev/scripts/shared/deps/outdated.sh

# === Testing ===

test:
    @~/code/.dev/scripts/shared/test/quick.sh

coverage:
    @~/code/.dev/scripts/shared/test/coverage.sh

# === Git ===

changes *args:
    @~/code/.dev/scripts/shared/git/changes.sh {{args}}

pre-commit:
    @~/code/.dev/scripts/shared/git/pre-commit.sh

# === Info ===

tree:
    @~/code/.dev/scripts/shared/info/tree.sh

loc:
    @~/code/.dev/scripts/shared/info/loc.sh

docs *args:
    @~/code/.dev/scripts/shared/info/docs.sh {{args}}

# === Run ===

run *args:
    @cargo run -- {{args}}
