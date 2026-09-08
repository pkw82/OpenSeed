# OpenSeed

Minimalny generator 24-słowowego angielskiego mnemonika BIP-39.

## Status Stage 7

Wersja v0.1 korzysta z systemowego CSPRNG i biblioteki `bip39`. Stage 4 dodaje
opcjonalne zbieranie ruchu myszy/touchpada przez obszar wskaźnika w GUI. Można
wybrać systemowy RNG, ruch albo oba źródła. Ruch jest oznaczony jako
eksperymentalny: liczba próbek nie jest przedstawiana jako liczba bitów
entropii. GUI pokazuje aktywność ruchu, licznik próbek i postęp do technicznego
progu 1024 próbek. Stage 5 dodaje opcjonalne wyniki fizycznych kości
sześciennych wprowadzane przyciskami 1–6. Przetwarzanie używa odrzucania
zakresu nadmiarowego zamiast modulo bias. Stage 6 dodaje wyłącznie informacyjny
panel o passphrase; OpenSeed nie przyjmuje ani nie przechowuje passphrase i nie
wykonuje jej derivacji. Kontroler nie jest jeszcze zaimplementowany. Stage 3 dodaje minimalne GUI oparte na
Slint z układem
responsywnym i systemowym renderowaniem czcionek; uruchomienie bez argumentów
otwiera okno, a `--cli` zachowuje tryb terminalowy.
Stage 7 dodaje osobny tryb weryfikacji 24 słów, listy angielskiej BIP-39 i
checksumu, bez zapisywania lub kopiowania wpisanego mnemonica.
Lokalne wyniki przygotowania release są opisane w [RELEASE.md](RELEASE.md);
publiczna dystrybucja wymaga jeszcze niezależnego audytu bezpieczeństwa,
audytu podatności zależności i podpisu dystrybucyjnego.

GUI pokazuje informacyjny status `ONLINE`/`OFFLINE` na podstawie lokalnych
interfejsów sieciowych i zaleca odłączenie Internetu przed generowaniem.
Generowanie nie jest blokowane przez aktywne połączenie sieciowe. Program sam
nie wykonuje jednak żadnych połączeń sieciowych.

## Uruchomienie

Wymagany jest Rust i Cargo:

```sh
cargo run --release
```

Pomoc:

```sh
cargo run --release -- --help
```

GUI wyświetla ostrzeżenia przed i po generowaniu, w tym informację, że po
zamknięciu programu mnemonic zostanie utracony bez możliwości odzyskania. Nie zapisuje, loguje, kopiuje
do schowka ani wysyła wygenerowanego sekretu. Publiczne adresy dotacji można
zaznaczyć i skopiować wyłącznie z panelu wsparcia; panel rozróżnia wpłatę
Bitcoin w BTC i wpłatę Lightning w satoshi. Użytkownik powinien
przepisać mnemonic offline i bezpiecznie przechować. Tryb terminalowy jest
dostępny przez:

```sh
cargo run --release -- --cli
```

Użytkownik odpowiada za odłączenie komputera od sieci, jeśli wymaga tego jego
procedura bezpieczeństwa. OpenSeed nie zapewnia już wymuszenia trybu offline.

## Licencja i odpowiedzialność

OpenSeed jest udostępniany na licencji MIT. Pełny tekst licencji znajduje się
w pliku [LICENSE-MIT](LICENSE-MIT).

Program jest dostarczany w stanie „takim, jaki jest”, bez gwarancji, że
spełni konkretny cel użytkownika albo zapewni dostępność lub odzyskanie
środków. Użytkownik sam odpowiada za ocenę środowiska, odłączenie sieci,
bezpieczne zapisanie mnemonica, ochronę urządzenia oraz poprawne użycie
programu i wygenerowanych słów. Utrata, ujawnienie, błędne przepisanie lub
nieprawidłowe użycie mnemonica może spowodować nieodwracalną utratę dostępu
do środków. OpenSeed nie przechowuje kopii mnemonica i nie może go odzyskać.

Informacja ta nie zastępuje niezależnego audytu bezpieczeństwa ani porady
prawnej.

## Testy

```sh
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

Testy obejmują 24 słowa, członkostwo każdego słowa w angielskiej liście BIP-39,
checksum wyliczony zgodnie ze specyfikacją oraz oficjalny wektor dla zerowej
entropii. Obejmują także odrzucanie nadmiarowych bloków wyników kości i
generowanie 256-bitowego wejścia bez modulo bias. Referencje: [BIP-39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki)
i [official English wordlist](https://github.com/bitcoin/bips/blob/master/bip-0039/english.txt).
