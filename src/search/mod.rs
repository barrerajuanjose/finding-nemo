pub fn get_host_by_site(site: String) -> String {
    format!("https://api.mercadolibre.com/sites/{}/search", site)
}