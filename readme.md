# Rust Template

This repo automates some quality of life improvements for generative artists. Every time you run your code:

- We create a random number generator seeded with the number of nanoseconds since 1970.
- We create a new folder inside the `checkpoints` directory and back up the current contents of your `src` folder there. That folder also has a file called `seed` that contains the seed used for this run.
- The image you created is saved to the `images` folder, losslesly compressed, and symlinked into its checkpoint folder.

This approach is way simpler than git and doesn't use much space. If you combine it with Dropbox, you get a super friendly backup system with 2 TB of storage for \$10/month.

Other advantages:

- Much generative art has an element of randomness. You can take advantage of that! Once you're pretty happy with a piece, make sure that it scales up smoothly, then change the argument of `loop_ntimes` in `main.rs` to some high number like 500. Our code will automatically save every run for you to curate later.

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

# Running a checkpoint

You can run a checkpoint like this:

```
scripts/run_checkpoint 2021-02-14 13:56:50 1
```

where `2021-02-14 13:56:50 1` is the name of the checkpoint folder. You don't need to wrap it in quotes or anything.
