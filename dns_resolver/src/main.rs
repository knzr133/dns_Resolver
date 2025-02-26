use std::net::IpAddr;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::Resolver;

fn main() {
    // Домен, який ми хочемо розв'язати
    let domain_name = "google.com";

    // Створюємо Resolver (клієнт для запитів DNS)
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default())
        .expect("Не вдалося створити Resolver");

    // Виконуємо DNS-запит
    match resolver.lookup_ip(domain_name) {
        Ok(response) => {
            println!("IP-адреси для {}:", domain_name);
            for ip in response.iter() {
                println!("{}", ip);
            }
        }
        Err(e) => {
            eprintln!("Помилка при запиті DNS: {}", e);
        }
    }
}
