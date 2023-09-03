# War (card game)

War (card game) simulation application. Application simulates a [War](<https://en.wikipedia.org/wiki/War_(card_game)>) card game, typically played by two players using a standard playing [card deck](https://en.wikipedia.org/wiki/Standard_52-card_deck).

The objective of the game is to win all the cards in the deck.

Here are the rules for playing the War card game:

1. **Setup:** The deck is evenly divided between the two players, with each player receiving 26 cards. It is important to keep your cards facedown and not to look at them.

2. **Gameplay:** Both players simultaneously reveal the top card from their respective decks and place them face-up on the table. The player with the higher-ranking card wins the round and collects both cards, placing them face-down at the bottom of their own pile. The ranking order of cards, from highest to lowest, is as follows: Ace, King, Queen, Jack, 10, 9, 8, 7, 6, 5, 4, 3, 2.

3. **War:** If the two revealed cards have the same rank, it results in a "**war**". In this case, both players place additional cards facedown and then reveal the next card from their decks. The player with the higher-ranking card wins all the cards on the table. If there is another tie, the process repeats until one player has a higher-ranked card.

4. **Card Value:** It's important to note that in the game of War, suits (hearts, diamonds, clubs, spades) do not affect the ranking or outcome of the game. Only the value of the card is considered.

5. **Winning:** The game continues until one player has collected all the cards, making them the winner. If a player runs out of cards during a **war**, they lose the game.

War is a simple and luck-based card game that is often played by children or as a casual pastime. It requires no skill or strategy, as the outcome of each round is solely determined by the card rankings.

## Usage

It is a command line application which simulates whole game. User can select a seed for deck shuffling and output mode.

Card deck is the standard 52-cards French-suited deck which comprises 13 ranks in each of the four suits: clubs (♣), diamonds (♦), hearts (♥) and spades (♠). The order in the initial deck (before shuffling) is following:

🃒  🃂  🂲  🂢  🃓  🃃  🂳  🂣  🃔  🃄  🂴  🂤  🃕  🃅  🂵  🂥  🃖  🃆  🂶  🂦  🃗  🃇  🂷  🂧  🃘  🃈  🂸  🂨  🃙  🃉  🂹  🂩  🃚  🃊  🂺  🂪  🃛  🃋  🂻  🂫  🃝  🃍  🂽  🂭  🃞  🃎  🂾  🂮  🃑  🃁  🂱  🂡

In the first step deck is shuffled using user specified or randomly generated seed.

### Example

```bash
$ war_card_game --seed 10
Starting game!
Deck: 🃒  🃂  🂲  🂢  🃓  🃃  🂳  🂣  🃔  🃄  🂴  🂤  🃕  🃅  🂵  🂥  🃖  🃆  🂶  🂦  🃗  🃇  🂷  🂧  🃘  🃈  🂸  🂨  🃙  🃉  🂹  🂩  🃚  🃊  🂺  🂪  🃛  🃋  🂻  🂫  🃝  🃍  🂽  🂭  🃞  🃎  🂾  🂮  🃑  🃁  🂱  🂡
Shuffling deck with seed 10
Deck: 🃎  🃖  🂺  🃘  🂥  🂭  🃙  🃁  🂱  🂲  🃝  🂸  🃑  🃂  🃓  🂶  🂾  🂤  🂡  🂵  🂫  🂮  🃅  🃊  🂽  🂣  🂦  🃉  🃚  🃋  🃞  🂻  🃔  🃒  🃕  🂷  🂢  🂩  🂨  🃈  🃍  🂴  🂧  🃇  🂪  🃃  🃛  🃆  🂹  🃄  🂳  🃗

Let's deal!
Starting setup:
P2: 🃗  🃄  🃆  🃃  🃇  🂴  🃈  🂩  🂷  🃒  🂻  🃋  🃉  🂣  🃊  🂮  🂵  🂤  🂶  🃂  🂸  🂲  🃁  🂭  🃘  🃖
P1: 🂳  🂹  🃛  🂪  🂧  🃍  🂨  🂢  🃕  🃔  🃞  🃚  🂦  🂽  🃅  🂫  🂡  🂾  🃓  🃑  🃝  🂱  🃙  🂥  🂺  🃎

Game over!
Player 1 won in round 146
Player 2 lost in 146
Longest war: 2
```

User can select multiple seeds by using seed range option

```bash
-r, --range <RANGE>    Range (start-end) of seeds for deck shuffling
```

where `RANGE` is in form of `start-end` and `start` < `end`. For seed range default (and only supported) output mode is `one-line` output level

```bash
Players: 2, seed: 0, winner: Player 2, winning round: 118
Players: 2, seed: 1, winner: Player 2, winning round: 176
Players: 2, seed: 2, winner: Player 2, winning round: 568
Players: 2, seed: 3, winner: Player 1, winning round: 654
Players: 2, seed: 4, winner: Player 2, winning round: 164
Players: 2, seed: 5, winner: Player 2, winning round: 126
Players: 2, seed: 6, winner: Player 2, winning round: 230
Players: 2, seed: 7, winner: Player 2, winning round: 116
Players: 2, seed: 8, winner: Player 1, winning round: 516
Players: 2, seed: 9, winner: Player 1, winning round: 380
```

User can select output level either by using output option

```bash
-o, --output <OUTPUT>  Output level [default: normal] [possible values: one-line, quiet, normal, verbose]
```

or "shortcut" options

```bash
  -v, --verbose          Verbose mode
  -q, --quiet            Quiet mode
  -1, --one-line         One line output mode
```

* `verbose` output level prints all the details of whole simulation

    ```bash
    war_card_game --seed 10 -v
    Starting game!
    Deck: 🃒  🃂  🂲  🂢  🃓  🃃  🂳  🂣  🃔  🃄  🂴  🂤  🃕  🃅  🂵  🂥  🃖  🃆  🂶  🂦  🃗  🃇  🂷  🂧  🃘  🃈  🂸  🂨  🃙  🃉  🂹  🂩  🃚  🃊  🂺  🂪  🃛  🃋  🂻  🂫  🃝  🃍  🂽  🂭  🃞  🃎  🂾  🂮  🃑  🃁  🂱  🂡
    Shuffling deck with seed 10
    Deck: 🃎  🃖  🂺  🃘  🂥  🂭  🃙  🃁  🂱  🂲  🃝  🂸  🃑  🃂  🃓  🂶  🂾  🂤  🂡  🂵  🂫  🂮  🃅  🃊  🂽  🂣  🂦  🃉  🃚  🃋  🃞  🂻  🃔  🃒  🃕  🂷  🂢  🂩  🂨  🃈  🃍  🂴  🂧  🃇  🂪  🃃  🃛  🃆  🂹  🃄  🂳  🃗

    Let's deal!
    Starting setup:
    P1: 🃗  🃄  🃆  🃃  🃇  🂴  🃈  🂩  🂷  🃒  🂻  🃋  🃉  🂣  🃊  🂮  🂵  🂤  🂶  🃂  🂸  🂲  🃁  🂭  🃘  🃖
    P2: 🂳  🂹  🃛  🂪  🂧  🃍  🂨  🂢  🃕  🃔  🃞  🃚  🂦  🂽  🃅  🂫  🂡  🂾  🃓  🃑  🃝  🂱  🃙  🂥  🂺  🃎

    Round 1
    P1: 🃗  🃄  🃆  🃃  🃇  🂴  🃈  🂩  🂷  🃒  🂻  🃋  🃉  🂣  🃊  🂮  🂵  🂤  🂶  🃂  🂸  🂲  🃁  🂭  🃘  🃖
    P2: 🂳  🂹  🃛  🂪  🂧  🃍  🂨  🂢  🃕  🃔  🃞  🃚  🂦  🂽  🃅  🂫  🂡  🂾  🃓  🃑  🃝  🂱  🃙  🂥  🂺  🃎
    P1 🃗  P2 🂳
    Round 1 winner is Player 1

    ...

    Round 146
    P1: 🃂
    P2: 🃖  🃞  🂭  🃝  🂲  🂶  🃄  🂻  🂸  🂺  🂴  🂧  🂳  🂽  🂥  🃘  🃃  🃁  🃊  🂡  🃈  🂩  🂨  🂾  🃚  🃗  🃔  🃑  🃍  🂦  🃆  🂱  🂵  🃛  🃒  🂮  🃋  🃎  🂹  🃕  🂤  🂫  🃙  🃅  🂢  🂪  🃉  🂷  🃓  🃇  🂣
    P2 🃖  P1 🃂
    Round 146 winner is Player 2

    Game over!
    Player 2 won in round 146
    Player 1 lost in 146
    Longest war: 2
    ```

* `normal` output level prints shuffling seed, initial and shuffled decks, starting hands of each player and the game result with final round number and the "longest" **war** (see [Example](#example)).
* `quiet` output level prints shuffling seed, shuffled deck and game result with final round number

    ```bash
    $ war_card_game --seed 10 -q
    Shuffling deck with seed 10
    Deck: 🃎  🃖  🂺  🃘  🂥  🂭  🃙  🃁  🂱  🂲  🃝  🂸  🃑  🃂  🃓  🂶  🂾  🂤  🂡  🂵  🂫  🂮  🃅  🃊  🂽  🂣  🂦  🃉  🃚  🃋  🃞  🂻  🃔  🃒  🃕  🂷  🂢  🂩  🂨  🃈  🃍  🂴  🂧  🃇  🂪  🃃  🃛  🃆  🂹  🃄  🂳  🃗

    Player 2 won in round 146
    Player 1 lost in 146
    ```

* `one-line` output level prints in single line same information as in `quiet` mode except shuffled deck

    ```bash
    $ war_card_game --seed 10 -1
    Players: 2, seed: 10, winner: Player 2, winning round: 146
    ```

## Story

Project was created as a Rust learning exercise after winter holiday party where my teenage son was playing this with (also teenage) son of my friend. It was fun watching they play it. I've joined in the middle and played as a third player for a few rounds but lost all of my cards and they continued for around next half of an hour or so.

We've started to wonder how long such games may take?

_If we assume one round takes ~2 seconds a game (without preparation) can take from less than a minute (seed 53822) to more than an hour (seed 6958)._

---

BartoszCiesla @ 2023
