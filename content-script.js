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

WebAssembly.instantiateStreaming(fetch(browser.runtime.getURL("wasm_link_color_bg.wasm")), {})
.then(results => {
    console.log("wasm returned", results.instance.exports.add(41, 3));
}).catch((err) => {
  console.error("Unable to instantiateStreaming", err)
});

// In JavaScript, the pattern `(async () => { <code> })()` is an
// immediately invoked function expression (IIFE) that is used to
// execute an asynchronous function immediately after defining it.

// An IIFE is simply a function that is wrapped in parentheses and
// invoked immediately after definition. The basic structure of an
// IIFE looks like this:

// ```javascript
// (function() {
//   // Your code here
// })();
// ```

// - `function() { ... }`: This is an anonymous function, which means
// - that it doesn't have a name and can't be invoked later through a
// - name.

// - `( ... )`: The pair of parentheses enclosing the anonymous
// - function serves to convert it into a function expression. Without
// - the surrounding parentheses, the JavaScript engine would
// - interpret it as a function declaration, which requires a name.

// - `(...)()`: The last set of parentheses is responsible for
// - invoking the function immediately after it has been defined.


// 1. `async () => { <code> }`: This creates an anonymous asynchronous
// arrow function. Any code within `{ <code> }` is executed in an
// asynchronous manner.

// 2. `(...)()`: The parentheses help create a function expression and
// the following `()` will immediately invoke that function
// expression. It means, as soon as the function is defined, it is
// called and executed.





