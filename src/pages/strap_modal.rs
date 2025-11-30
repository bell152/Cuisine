use stylist::yew::styled_component;
use yew::prelude::*;

use web_sys::HtmlElement;
use js_sys::eval;

#[styled_component(BootstrapModal)]
pub fn bootstrap_modal() -> Html {
   


    // eval函数在浏览器中有执行漏洞,使用Yew 的 Callback 特性
    let node_ref = use_node_ref();
    use_effect_with(node_ref.clone(), move |node_ref| {
        if let Some(element) = node_ref.cast::<HtmlElement>() {
           /*  let modal = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("exampleModal")
                .unwrap(); */

                let script = format!(
                    r#"
                    var myModal = new bootstrap.Modal(document.getElementById('{}'));
                    myModal.show();
                    "#,
                    element.id()
                );
                if let Err(err) = eval(&script) {
                    web_sys::console::error_1(&err);
                }
        }
        || ()
    }); 

   /*  use_effect_with(node_ref.clone(), move |node_ref| {
        if let Some(element) = node_ref.cast::<HtmlElement>() {
            let modal_id = element.id();
            let document = web_sys::window().unwrap().document().unwrap();
            let script = document.create_element("script").unwrap();
            script.set_inner_html(&format!(
                r#"
                var myModal = new bootstrap.Modal(document.getElementById('{}'));
                myModal.show();
                "#,
                modal_id
            ));
            document.body().unwrap().append_child(&script).unwrap();
        }

        || ()
    });  */
   
    html! {
        <>
        <div class="modal fade blue-100" id="exampleModal" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true" ref={node_ref}>
            <div class="modal-dialog ">
                <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title blue-100" id="exampleModalLabel">{ "Modal title" }</h5>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                    </div>
                    <div class="modal-body">
                        { "This is a Bootstrap modal!" }
                    </div>
                    <div class="modal-footer">
                        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{ "Close" }</button>
                        <button type="button" class="btn btn-primary">{ "Save changes" }</button>
                    </div>
                </div>
            </div>
        </div>
        </>
    }
}