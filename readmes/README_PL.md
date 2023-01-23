<div align="center">
<img src="https://raw.githubusercontent.com/Gojodzojo/turing-machine/main/icon.ico" alt="icon" width="140px" />

# Maszyna Turinga
</div>

Jest to program symulujący działanie [maszyny Turinga](https://pl.wikipedia.org/wiki/Maszyna_Turinga). Program ten został wykonany jako projekt zaliczeniowy na zajęcia z programowania na Politechnice Śląskiej.

## Działanie Maszyny Turinga
Maszyna Turinga składa się z czterech części: 
* taśmy ze znakami,
* tablicy charakterystycznej,
* głowicy (kursora),
* wewnętrznego stanu maszyny.

Podczas działania maszyny, w pierwszej kolejności głowica odczytuje znak nad którym się aktualnie znajduje. Następnie sprawdza swój wewnętrzny stan (na początku zawsze jest to 0) i na podstawie odczytanego znaku oraz swojego stanu, wybiera komórkę z tablicy charakterystycznej.

Każda taka komórka zawiera 3 informacje: 
* nowy stan maszyny,
* nowy znak na taśmie,
* kierunek ruchu głowicy.

Na podstawie informacji z komórki w miejsce odczytanego wcześniej znaku zostaje wpisany nowy znak, potem maszyna zmienia swój stan na nowy, a na koniec głowica przesuwa się w prawo lub w lewo. 

Wszystkie powyższe czynności powtarzane są w pętli dopóki nie nastąpi jeden z poniższych przypadków: 
* w tablicy charakterystycznej nie istnieje komórka określona znakiem i stanem,
* w komórce jako kierunek ruchu podane zostało 0,
* ruch głowicą wymagałby wyjechania poza taśmę.

## Interfejs użytkownika
Po otworzeniu programu widoczne są dwie części: lewa kolumna z ikoną i prawa z symulatorem.

### Lewa kolumna
Kolumna po lewej stronie zawiera przyciski umożliwiające kolejno: 
* utworzenie nowego pliku,
* otwarcie zapisanego wcześniej pliku,
* zapisanie pliku,
* zapisanie pliku jako nowy plik,

Kolumna po lewej stronie umożliwia również dostosowanie ustawień takich jak:
* język aplikacji,
* motyw aplikacji.

Kolumnę po lewej stronie można otworzyć lub zamknąć za pomocą przycisku u góry po prawej stronie od linii kolumny.

### Symulator
W trybie edycji (domyślnym) po lewej stronie można dostosować ustawienia takie jak: 
* tekst taśmy,
* długość taśmy,
* pozycja kursora (głowicy),
* liczba stanów tablicy,
* znaki tablicy.

Po prawej stronie znajduje się tabela w której można wpisywać wartości poszczególnych komórek. Wartości te są ustawione w następującej kolejności: 
* nowy stan maszyny (od 0 do 99),
* nowy znak na taśmie,
* kierunek ruchu głowicy (`+`, `-` lub `0`).

U góry znajduje się podgląd początkowych znaków taśmy.

Po ustawieniu wszystkich parametrów można przejść do trybu symulacji klikając przycisk Start.

Po lewej stronie znajdują się: 
* informacja o liczbie wykonanych kroków,
* informacja o wewnętrznym stanie maszyny,
* suwak do zmieniania interwału samowyzwalacza maszyny,
* przycisk do ręcznego zmieniania kroków,
* przycisk Stop do powrotu w tryb edycji.

Po prawej stronie znajduje się tabela w której wyświetlają się ustawione wcześniej wartości komórek.

U góry znajduje się podgląd obecnego stanu taśmy.

## Skróty klawiszowe
* `tab` = Przełącz pole tekstowe,
* `ctrl` + `s` = Zapisz plik,
* `ctrl` + `+` = Przybliż widok,
* `ctrl` + `-` = Oddal widok.

## Przykłady
[Tutaj](https://github.com/Gojodzojo/turing-machine/tree/main/examples) znajdują się pliki z przykładowymi tablicami charakterystycznymi maszyny Turinga. Niektóre z nich były dołączone do innego symulatora maszyny Turinga.

## Samodzielne kompilowanie
By skompilować ten program należy zainstalować kompilator języka Rust zgodnie z [tą instrukcją](https://www.rust-lang.org/tools/install). Następnie należy sklonować to repozytorium, wejść do niego w terminalu i wykonać komendę: 
```
cargo run --release
```
Skompilowany program będzie znajdował się w folderze `target/release`.


## Ciekawostka
Ikona tego programu została stworzona przez generator [DALL·E 2](https://openai.com/dall-e-2/).