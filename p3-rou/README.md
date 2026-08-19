# p3-rou

Library and CLI for Patrician 3 trade route files (`.rou`, stored in `Save/AutoRoute`).

The library covers the file format (stop serialization, decompression) and route
construction: the `builder` module holds the verified format semantics (operation sign
encodings, flag bytes, the first-stop marker, the instruction-order array, the MAX
sentinel) and assembles the route types below, so mods can build routes in-game from
the same code the CLI uses.

The game saves routes compressed. The compression algorithm has not been reverse engineered, so generated files are written
uncompressed (negative length header). The vanilla game fails to load uncompressed routes — the
`mod-fix-uncompressed-trade-route-loading` mod fixes that and must be installed to use generated files.

## Dump a route

```
roucli dump "Save/AutoRoute/route.rou"
```

Handles both compressed (game-saved) and uncompressed (generated) files.

## Generate a route

```
roucli generate route.toml -o "Save/AutoRoute/route.rou"
```

Route description format:

```toml
[[stops]]
town = 0x0a  # town index within the savegame's town list, see below
flag = "R"   # optional stop flag as shown in the route window: "R" (repair), "X" (default), or "-"

[[stops.load]] # transfer from trading office onto the ship
ware = "Fish"
amount = 2     # in-game units (Last/barrels/pieces); omit for "as much as possible"

[[stops]]
town = 0x11

[[stops.sell]] # sell to the town
ware = "Fish"
amount = 2
min_price = 700

[[stops.unload]] # transfer from the ship into the trading office
ware = "Fish"

[[stops.buy]] # buy from the town
ware = "Beer"
amount = 20
max_price = 43
```

Ware names are the exact `WareId` identifiers of `p3-api`: Grain, Meat, Fish, Beer, Salt, Honey,
Spices, Wine, Cloth, Skins, WhaleOil, Timber, IronGoods, Leather, Wool, Pitch, PigIron, Hemp, Pottery, Bricks, Sword, Bow,
Crossbow, Carbine.

Town indices are savegame-specific (towns founded during a game shift the list). To find the index of a town, save a route
that stops there in-game and inspect it with `roucli dump`.

## Write a route from the reference goods table

```
roucli write --type 5stop --citizens 2500 --load-town 0x0a --sell-town 0x11 -o "Save/AutoRoute/supply.rou"
```

Generates a supply route from a reference goods table. Quantities are scaled linearly from the reference citizens count
(rounding up), prices are used as-is. Every type loads the calculated quantities at the source town (with the repair flag
set), sells the reference goods at the given minimum prices in the target town, resets the target office stock to exactly
the calculated quantities, and unloads the whole ship back into the source office.

- `--type 5stop`: source load → sell max → take the entire target office stock → put back the calculated quantities →
  unload everything at the source.
- `--type 6stop`: like 5stop, but swaps the target office stock one unit category at a time (unload loads-goods + take
  barrels, then unload barrel quantities + take loads-goods, then unload loads quantities), which bounds how much ship
  space the shuffle needs.
- `--type suck`: parks in the load town (`--sell-town` and `--citizens` are unused): one stop unloading everything into
  the office (with the repair flag), then five stops buying all goods at the reference `buy_price` limits.

The stops use town indices 0 unless `--load-town`/`--sell-town` are given (savegame-specific, find them with `dump`).
Towns of load/unload stops need a trading office — the game wipes office transfers in office-less towns at route load
time.

The built-in reference table ships in `src/supply_reference.toml`; pass `--reference <file>` to use a custom one, and
`--citizens` defaults to 1000. The table holds data for all trade wares: `supply` (quantity per `citizens` inhabitants;
goods without it are not part of citizens' supply), `sell_price` (minimum sell price used by supply routes) and
`buy_price` (maximum buy price, used by the suck type). Ware keys are the exact `WareId` identifiers:

```toml
citizens = 1000

[goods]
Beer = { supply = 28, sell_price = 49, buy_price = 38 }
Bricks = { sell_price = 120, buy_price = 77 }
```
