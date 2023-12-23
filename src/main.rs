use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ImageProps {
    pub current_index: i32,
    pub message: Callback<Message>
}

pub enum Message {
    Next,
    Prev
}

#[function_component(Carousel)]
fn image_carousel(props: &ImageProps) -> Html {
    let props_clone = props.clone();
    let props_clone_next = props.clone();
    let images: Vec<String> = vec![
        "https://abcbiznes.ru/wp-content/uploads/2015/04/pivovarnya.jpg".to_owned(),
        "https://mwbeer.com.ua/wp-content/uploads/2018/09/DSC_6977.jpg".to_owned(),
        "https://7d9e88a8-f178-4098-bea5-48d960920605.selcdn.net/2b1d2d07-61fe-4727-91d4-ae4845231d7c/-/resize/x833/-/quality/lighter/".to_owned()
    ];
    html! {
        
            <div class="carousel">
                <div class="image-container">
                { for images.iter().enumerate().map(|(index, url)| {
                    if index == props.current_index as usize {
                        html! { <img src={url.to_string()}/> }
                    }
                    else { html! {} }
                })}
                </div>
                <div class="controls">
                    <button class = "carousel-button" onclick={move |_| props_clone.message.emit(Message::Prev)}>{ "<" }</button>
                    <button class = "carousel-button" onclick={move |_| props_clone_next.message.emit(Message::Next)}>{ ">" }</button>
                </div>
            </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let image_index = use_state(|| 0);
    let image_count = 3;
    let prop_callback = {
        let image_index = image_index.clone();
        Callback::from(move |message: Message| {
            match message {
                Message::Next => {
                    if *image_index < image_count - 1{
                        image_index.set(*image_index + 1);
                    }
                    else if *image_index == image_count - 1 { image_index.set(0); }
                },
                Message::Prev => {
                    if *image_index > 0 {
                        image_index.set(*image_index - 1);
                    }
                    else if *image_index == 0 {
                        image_index.set(image_count - 1);
                    }
                }
            }
        })
    };
    html! {
        <>
            <header>
            <h1>{ "Karav Brewery" }</h1>
                <nav>
                    <ul>
                        <li><a href="#"> { "О нас" }</a></li>
                        <li><a href="#">{ "Контакты" }</a></li>
                        <li><a href="#">{ "Магазин" }</a></li>
                    </ul>
                </nav>
            </header>

            <main>
                <Carousel current_index={*image_index} message={prop_callback} />
            </main>

            <footer>
                <p>{ "Карав Бревери © 2023" }</p>
                <p>{ "Адрес: ул. Пивоварная, 1, Город" }</p>
                <p>{ "Телефон: +7 (123) 456-7890" }</p>
            </footer>
        </>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
