# druid-components

A collection of custom components in [Druid](https://github.com/linebender/druid).
NOTE: This crate is WIP, some api may removed in next commit and other api may be added.

## Components

Include: 

- Material-Design Style
- Hui-Works Style

## Notice For Using This Crate

We follow [Material Design's Color System](https://material.io/design/color/the-color-system.html),so please configure your own color system in `AppLauncher`, or use the default config. **(will be `panic!` if CONFIGURE nothing!!)**

There is a example for configure color.

```rust
...

   AppLauncher::with_window(window).configure_env(|env, _| {
      
        env.set(key::PRIMARY, Color::Rgba32(0x6200EEFF));

        env.set(key::PRIMARY_VARIANT, Color::Rgba32(0x3700B3FF));

        env.set(key::SECONDARY, Color::Rgba32(0x03DAC6FF));

        env.set(key::SECONDARY_VARIANT, Color::Rgba32(0x018786FF));

        env.set(key::BACKGROUND, Color::Rgba32(0xFFFFFFFF));

        env.set(key::SURFACE, Color::Rgba32(0xFFFFFFFF));

        env.set(key::ON_PRIMARY, Color::Rgba32(0xFFFFFFFF));

        env.set(key::ON_SECONDARY, Color::Rgba32(0x000000FF));

        env.set(key::ON_BACKGROUND, Color::Rgba32(0x000000FF));

        env.set(key::ON_SURFACE, Color::Rgba32(0x000000FF));

        env.set(key::ON_ERROR, Color::Rgba32(0xFFFFFFFF));
        
        }
    );
   ... 
```

Or just use the default color config...
(maybe we can add mor pre-config color config in next commit)

```rust
...

   AppLauncher::with_window(window).configure_env(|env, _| {
      // default color system will be used.
      druid_components::env::color::setup_default_env(env);
        }
    );
   ... 
```

## Thanks

- [druid-material](https://github.com/Maan2003/druid-material)