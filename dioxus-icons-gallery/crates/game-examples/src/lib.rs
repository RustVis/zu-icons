use dioxus::prelude::*;

mod andymeneely_page;
mod aussiesim_page;
mod badges_page;
mod carl_olsen_page;
mod caro_asercion_page;
mod cathelineau_page;
mod catsu_page;
mod darkzaitzev_page;
mod delapouite_page;
mod faithtoken_page;
mod felbrigg_page;
mod generalace135_page;
mod guard13007_page;
mod heavenly_dog_page;
mod irongamer_page;
mod john_colburn_page;
mod john_redman_page;
mod kier_heyl_page;
mod lorc_page;
mod lord_berandas_page;
mod lucasms_page;
mod pepijn_poolman_page;
mod pierre_leducq_page;
mod priorblue_page;
mod quoting_page;
mod rihlsul_page;
mod sbed_page;
mod seregacthtuf_page;
mod skoll_page;
mod sparker_page;
mod spencerdub_page;
mod starseeker_page;
mod various_artists_page;
mod viscious_speed_page;
mod willdabeast_page;
mod zajkonur_page;
mod zeromancer_page;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        h1 { "Bootstrap icons for Dioxus" }

        h2 { "Aussiesim icons" }
        aussiesim_page::AussiesimPage {}

        h2 { "Badges icons" }
        badges_page::BadgesPage {}

        h2 { "CarlOlsen icons" }
        carl_olsen_page::CarlOlsenPage {}

        h2 { "CaroAsercion icons" }
        caro_asercion_page::CaroAsercionPage {}

        h2 { "Cathelineau icons" }
        cathelineau_page::CathelineauPage {}

        h2 { "Catsu icons" }
        catsu_page::CatsuPage {}

        h2 { "Darkzaitzev icons" }
        darkzaitzev_page::DarkzaitzevPage {}

        h2 { "Delapouite icons" }
        delapouite_page::DelapouitePage {}

        h2 { "Faithtoken icons" }
        faithtoken_page::FaithtokenPage {}

        h2 { "Felbrigg icons" }
        felbrigg_page::FelbriggPage {}

        h2 { "Generalace135 icons" }
        generalace135_page::Generalace135Page {}

        h2 { "Guard13007 icons" }
        guard13007_page::Guard13007Page {}

        h2 { "HeavenlyDog icons" }
        heavenly_dog_page::HeavenlyDogPage {}

        h2 { "Irongamer icons" }
        irongamer_page::IrongamerPage {}

        h2 { "JohnColburn icons" }
        john_colburn_page::JohnColburnPage {}

        h2 { "JohnRedman icons" }
        john_redman_page::JohnRedmanPage {}

        h2 { "KierHeyl icons" }
        kier_heyl_page::KierHeylPage {}

        h2 { "Lorc icons" }
        lorc_page::LorcPage {}

        h2 { "LordBerandas icons" }
        lord_berandas_page::LordBerandasPage {}

        h2 { "Lucasms icons" }
        lucasms_page::LucasmsPage {}

        h2 { "PepijnPoolman icons" }
        pepijn_poolman_page::PepijnPoolmanPage {}

        h2 { "PierreLeducq icons" }
        pierre_leducq_page::PierreLeducqPage {}

        h2 { "Priorblue icons" }
        priorblue_page::PriorbluePage {}

        h2 { "Quoting icons" }
        quoting_page::QuotingPage {}

        h2 { "Rihlsul icons" }
        rihlsul_page::RihlsulPage {}

        h2 { "Sbed icons" }
        sbed_page::SbedPage {}

        h2 { "Seregacthtuf icons" }
        seregacthtuf_page::SeregacthtufPage {}

        h2 { "Skoll icons" }
        skoll_page::SkollPage {}

        h2 { "Sparker icons" }
        sparker_page::SparkerPage {}

        h2 { "Spencerdub icons" }
        spencerdub_page::SpencerdubPage {}

        h2 { "Starseeker icons" }
        starseeker_page::StarseekerPage {}

        h2 { "VariousArtists icons" }
        various_artists_page::VariousArtistsPage {}

        h2 { "VisciousSpeed icons" }
        viscious_speed_page::VisciousSpeedPage {}

        h2 { "Willdabeast icons" }
        willdabeast_page::WilldabeastPage {}

        h2 { "Zajkonur icons" }
        zajkonur_page::ZajkonurPage {}

        h2 { "Zeromancer icons" }
        zeromancer_page::ZeromancerPage {}
    }
}
