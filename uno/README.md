Q Give me code for the simpelest content script based addon

The simplest content script-based Firefox add-on consists of two files: the content script itself (let's call it "content-script.js") and the manifest file that describes the addon (usually called "manifest.json").

1. Create "content-script.js" file with the following code:

```javascript
// This content script does nothing more than log a message to the console
console.log("Simple content script is running!");
```

2. Create the "manifest.json" file with the following content:

```json
{
  "manifest_version": 2,
  "name": "Simple Content Script Addon",
  "version": "1.0",
  "description": "A simple addon that runs a content script",
  "content_scripts": [
    {
      "matches": ["*://*/*"],        // This tells the addon to run the content script on all web pages
      "js": ["content-script.js"]    // This is the JavaScript file which contains the content script
    }
  ],
  "permissions": ["activeTab"]       // This permission is required to access the content of the active tab
}
```

Now you can test your simple content script Firefox add-on:

1. Open Firefox and type "about:debugging" in the address bar.
2. Click on "This Firefox" in the sidebar.
3. Click on "Load Temporary Add-on...," find the folder with your "manifest.json" and "content-script.js" files and open the "manifest.json" file.
4. The add-on is now temporarily loaded until you restart the browser. Navigate to any web page and open the browser console (Ctrl+Shift+K). You should see the message "Simple content script is running!" logged to the console.



-----------------------

Content scripts can directly interact with the DOM (Document Object Model) of the web page they're injected into, just like regular JavaScript running on the page. Here's an example of how a content script can examine and manipulate a web page:

Create a file named `content-script.js` with the following content:

```javascript
// Log the page title
console.log("Page title:", document.title);

// Find all the paragraphs on the web page
const paragraphs = document.querySelectorAll('p');
console.log(`There are ${paragraphs.length} paragraphs on this page.`);

// Change the text color of all links to red
const links = document.querySelectorAll('a');
for (const link of links) {
  link.style.color = 'red';
}

// Inject a new heading at the top of the web page
const heading = document.createElement('h1');
heading.innerText = 'Injected by the content script';
document.body.insertBefore(heading, document.body.firstChild);
```

Update the `manifest.json` to use this new content script:

```json
{
  "manifest_version": 2,
  "name": "Examine Web Page Addon",
  "version": "1.0",
  "description": "A content script addon that examines and manipulates the web page",
  "content_scripts": [
    {
      "matches": ["*://*/*"],
      "js": ["content-script.js"]
    }
  ],
  "permissions": ["activeTab"]
}
```

-----------------------

Q: 2023-05-13T16:51:36
How can I call a Rust function from my context script?

To call a Rust function from your content script, you should compile your Rust code to WebAssembly (Wasm) first, then load and execute the Wasm module within your content script. Here's a step-by-step guide on how to do this:

Note: To follow this guide, make sure you've installed Rust and its WebAssembly target. If you haven't, visit https://www.rust-lang.org/tools/install to install Rust and run `rustup target add wasm32-unknown-unknown` to add the wasm target.

1. Create a Rust library project:
```sh
cargo new --lib rust_wasm_module
cd rust_wasm_module
```

2. Edit `Cargo.toml` to create a wasm target:
```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
```

3. Edit `src/lib.rs` to define a Rust function you want to call from your content script:
```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

4. Build the Rust library as Wasm:
```sh
cargo build --target wasm32-unknown-unknown --release
```

5. In the `rust_wasm_module` folder, install `wasm-bindgen-cli` (if you haven't already) and generate the necessary JavaScript bindings:

```sh
cargo install wasm-bindgen-cli
wasm-bindgen target/wasm32-unknown-unknown/release/rust_wasm_module.wasm --out-dir wasm_output
```

6. Copy `rust_wasm_module/wasm_output/rust_wasm_module_bg.wasm` and `rust_wasm_module/wasm_output/rust_wasm_module.js` files to the folder of your Firefox extension (where `manifest.json` and `content-script.js` are located).

7. Update the `content-script.js` of your Firefox extension to import the Wasm module and call the Rust function:

```javascript
(async () => {
  // Import the wasm module and its initialization function
  const rustWasmModule = await import("./rust_wasm_module.js");

  // Initialize and instantiate the wasm module
  const wasm = await rustWasmModule.default();

  // Call the Rust function 'add' from the wasm module
  const a = 42;
  const b = 13;
  const result = wasm.add(a, b);

  // Log the result to the console
  console.log(`${a} + ${b} = ${result}`); // Should log: "42 + 13 = 55"
})();
```

8. Update the `manifest.json` to allow fetching local Wasm modules:

```json
{
  "manifest_version": 2,
  "name": "Call Rust Function Addon",
  "version": "1.0",
  "description": "A content script addon that calls a Rust function compiled to WebAssembly",
  "content_scripts": [
    {
      "matches": ["*://*/*"],
      "js": ["content-script.js"]
    }
  ],
  "permissions": ["activeTab"],
  "web_accessible_resources": ["rust_wasm_module_bg.wasm"]
}
```

Now, follow the steps from previous answers to load the add-on in Firefox. The content script should call the Rust `add` function from the WebAssembly module and log the result to the console.

Be aware that the performance overhead caused by loading WebAssembly modules might not be ideal for very simple functions. Consider using WebAssembly for more computationally intensive tasks, where the performance benefits become more apparent.

