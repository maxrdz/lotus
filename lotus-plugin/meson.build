crate_name = 'lotus-plugin'
crate_type = 'rlib'

# ----- Cargo Build ----- #

crate_cargo_opts = [ '--package', crate_name ]
feature_flags = []

# Convert FF list to argument string for --features option
cargo_ff_arg = ''
first_flag = false
foreach ff : feature_flags
  if first_flag == false
    cargo_ff_arg += ff
    first_flag = true
  else
    cargo_ff_arg += ',' + ff
  endif
endforeach

if feature_flags.length() > 0
  crate_cargo_opts += [ '--no-default-features', '--features', cargo_ff_arg ]
else
  message('Building lotus-plugin with default feature flags.')
endif

if crate_type == 'bin'
  out_artifact = crate_name.replace('-', '_')
endif

if crate_type == 'cdylib'
  out_artifact = 'lib' + crate_name.replace('-', '_') + '.so'
endif

if crate_type == 'rlib'
  out_artifact = ''
endif

crate_build = custom_target(
  'lotus-plugin-build',
  build_by_default: true,
  build_always_stale: true,
  output: 'lotus-plugin-cargo-build',
  console: true,
  env: cargo_env,
  command: [
    cargo_bin, 'build', cargo_opts, crate_cargo_opts,
  ]
)

if out_artifact != ''
  # Replace below with `--artifact-dir` cargo option once stable.
  # Issue: https://github.com/rust-lang/cargo/issues/6790
  custom_target(
    'lotus-plugin',
    build_by_default: true,
    build_always_stale: true,
    output: binary,
    console: true,
    env: cargo_env,
    command: [
      'cp', target_dir / out_artifact, '@BUILD_ROOT@' / crate_name / out_artifact,
    ],
    depends: crate_build,
  )
endif
