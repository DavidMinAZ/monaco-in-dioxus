# Purpose

This project is intended as a brief proof-of-concept(?) to demonstrate the use of the Monaco editor within a Rust project using a Dioxus UI. This was _not_ intended to demonstrate creation of a full-blow code editor, but rather is being done to facilitate a familiar experience for users to enter and store code within an application which uses user-generated snippets in its operations.

# Development

Although Monaco seemed to have ample examples around the web for implenenting it it in various ways, I did not immediately surface any solutions for using it in Dioxus the way I had envisioned.

Reading the docs for both Dioxus and Monaco was an option, of course, and may have (in hindsight) been the better option. Instead, I leveraged multiple AI models to cobble together a basic working project as a working example of these things working together. 

Although this approach was rife with challenges due to many of the models (GPT-4/5, Claude Sonnet 3.7, Gemini, and maybe others -- I've lost track!) struggling with Dioxus 0.6, I was able to coax them into helping get Monaco to load (via CDN for now) and allow manually-triggered exchange of content from the editor component to Rust -- and back.

To that end, I offer this repo as my contribution to anyone else embarking on such an endeavor.


### Starting the dev/demo server

The project, as-is, was intended to address a __desktop__ target with potential future use with other targets.

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

