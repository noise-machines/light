# Rust Template

This repo automates some quality of life improvements for generative artists. Every time you run your code:

- We create a random number generator seeded with the number of nanoseconds since 1970.
- We create a new folder inside the `snapshots` directory and back up the current contents of your `src` folder there. That folder also has a file called `seed` that contains the seed used for this run.
- The image you created is saved to the `images` folder, losslesly compressed, and symlinked into its snapshot folder.

This approach is way simpler than git and doesn't use much space. If you combine it with Dropbox, you get a super friendly backup system with 2 TB of storage for \$10/month.

Other features:

- Much generative art has an element of randomness. You can take advantage of that! Once you're pretty happy with a piece, make sure that it scales up smoothly, then change the argument of `loop_ntimes` in `main.rs` to some high number like 500. Our code will automatically save every run for you to curate later.
- Run on save. Any time you save your editor in a Rust file, VS Code will automatically re-run your sketch.

# Getting started

Make sure you have Image Magick installed:

```
brew install imagemagick
```

Then start your piece by running:

```
cargo run --release
```

Your code and its output will be automatically saved 🎉

# Running a snapshot

You can run a snapshot like this:

```
scripts/run_snapshot 2021-02-14 13:56:50 1
```

where `2021-02-14 13:56:50 1` is the name of the snapshot folder. You don't need to wrap it in quotes or anything.
