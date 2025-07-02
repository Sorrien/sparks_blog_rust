use html_node::{Node, text, typed::elements::*, unsafe_text};

pub fn base_page(body: html_node::Node) -> Node {

    let script = r#"<script type="module"> import init, { add } from './pkg/hello_wasm_site.js'; async function run() { await init(); } run(); </script>"#;
    html_node::typed::html!(<!DOCTYPE html><html>
        <head>
          <meta charset="utf-8" />
          <meta name="viewport" content="width=device-width" />
          <title>Sparks Blog</title>
          <base href="/" />
          //<link href="https://stackpath.bootstrapcdn.com/bootstrap/4.4.1/css/bootstrap.min.css" rel="stylesheet" />
          <link href="styles/bootstrap.min.css" rel="stylesheet" />
          <link href="styles/normalize.css" rel="stylesheet" />
          <link href="styles/site.min.css" rel="stylesheet" />
          <link href="styles/font-awesome.min.css" rel="stylesheet" />
          //<link href="https://stackpath.bootstrapcdn.com/font-awesome/4.7.0/css/font-awesome.min.css" rel="stylesheet" />
        </head>
        
        <body> 
          
            // Use ES module import syntax to import functionality from the module
            // that we have compiled.
            //
            // Note that the `default` import is an initialization function which
            // will "boot" the module and make it ready to use. Currently browsers
            // don't support natively imported WebAssembly as an ES module, but
            // eventually the manual initialization won't be required!
            {unsafe_text!("{}", script)}
        
/*             async function run() {
              // First up we need to actually load the Wasm file, so we use the
              // default export to inform it where the Wasm file is located on the
              // server, and then we wait on the returned promise to wait for the
              // Wasm to be loaded.
              //
              // It may look like this: `await init('./pkg/without_a_bundler_bg.wasm');`,
              // but there is also a handy default inside `init` function, which uses
              // `import.meta` to locate the Wasm file relatively to js file.
              //
              // Note that instead of a string you can also pass in any of the
              // following things:
              //
              // * `WebAssembly.Module`
              //
              // * `ArrayBuffer`
              //
              // * `Response`
              //
              // * `Promise` which returns any of the above, e.g. `fetch("./path/to/wasm")`
              //
              // This gives you complete control over how the module is loaded
              // and compiled.
              //
              // Also note that the promise, when resolved, yields the Wasm module's
              // exports which is the same as importing the `*_bg` module in other
              // modes
              await init();
        
              // And afterwards we can use all the functionality defined in wasm.
              //const result = add(1, 2);
              //console.log(`1 + 2 = ${result}`);
              //if (result !== 3)
              //  throw new Error("wasm addition doesn't work!");
            }
        
            run(); */
          

          {body}
        
        </body>
        
        </html>)
}

pub fn main_layout(body: html_node::Node, current_year: i32, current_page: Option<usize>) -> Node {
    let navbar_pages = vec![("", "Home"), ("blog", "Blog"), ("about", "About")];

    let nav_menu = html_node::typed::html!(
        <nav class="navbar navbar-inverse container">
        <a class="navbar-brand" href="#">Collin Sparks Portfolio</a>
        <ul class="nav navbar-nav navbar-right">
        {
            navbar_pages.into_iter().enumerate().map(|(i, (href, label))| {
            let (active, aria_current) = if let Some(index) = current_page {
                if index == i {       
                (" active", " aria-current=\"page\"")
                }
                else {
                    ("", "")
                }
            }
            else {
                ("", "")
            };
             html_node::typed::html!(<li class="nav-item">
            {unsafe_text!("<a class=\"nav-link{0}\" href=\"{1}\"{2}> {3} </a>", active, href, aria_current, label)}
        </li>)
        })}
        </ul>
        </nav>);

    base_page(html_node::typed::html!(
            <div class="page">
    {nav_menu}
    <div class="navbar-underline"></div>
        <main>
            <article class="body-content">
                {body}
            </article>

            <footer id="footer-bottom" class="container-fluid">
                <div class="row">
                    <div class="col-md-6 offset-md-3">
                        <ul id="social-media" class="list-inline">
                            <li class="list-inline-item">
                                <a href="https://www.linkedin.com/in/collin-sparks-b6065953/" rel="noreferrer" target="_blank" alt="Linkedin">
                                    <span aria-hidden="true" class="fa-stack 1x">
                                        <i class="fa fa-circle fa-stack-2x media-icon-background"></i>
                                        <i class="fa fa-linkedin fa-stack-1x"></i>
                                    </span>
                                </a>
                            </li>
                            <li class="list-inline-item">
                                <a href="https://github.com/Sorrien" rel="noreferrer" target="_blank" alt="Github">
                                    <span aria-hidden="true" class="fa-stack 1x">
                                        <i class="fa fa-circle fa-stack-2x media-icon-background"></i>
                                        <i class="fa fa-github-alt fa-stack-1x"></i>
                                    </span>
                                </a>
                            </li>
                            <li class="list-inline-item">
                                <a href="/feed.xml" rel="noreferrer" target="_blank" alt="RSS">
                                    <span aria-hidden="true" class="fa-stack 1x">
                                        <i class="fa fa-circle fa-stack-2x media-icon-background"></i>
                                        <i class="fa fa-rss fa-stack-1x"></i>
                                    </span>
                                </a>
                            </li>
                        </ul>
                        <div class="copyright">
                            <p><span class="fa fa-copyright"></span> {text!("{current_year}")} - Collin Sparks </p>
                            <p id="disclaimer">Disclaimer: Opinions expressed are solely my own and do not express the views or opinions of my employer.</p>
                        </div>
                    </div>
                </div>
            </footer>
        </main>
    </div>
        ))
}
