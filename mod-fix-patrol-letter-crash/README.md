# mod-fix-patrol-letter-crash

Fixes a vanilla crash-to-desktop when opening the Personal letters list while it
contains certain scripted letters - in practice the patrol mission's "Patrol
destination" letters.

Install: put `fix_patrol_letter_crash.dll` into the `mods` folder (requires the
modloader).

## The bug

Every message carries a town byte that the letters list draws as its town column,
by indexing the 40-slot town-name text bank at `0x6dda00`. The lookup has no
bounds check (`0x47d928: mov eax, [edx*4+0x6dda00]`), and the resulting pointer
goes to ddraw_Dll's text draw, which dereferences it without any guard
(`ddraw_Dll+0xf100`).

The letter-creation script command (handler `0x4ed4a0`) fills that town byte with
the low byte of a script variable, unvalidated (`0x4ed4e4`), and the patrol
mission asks it for a variable that does not exist. The missions are script files
inside the archives, and command 37 of `missions_addon/patrouille.p2m` - the
"Patrol destination" letter - names **variable 131** in a script that declares 25
variables. The array is `malloc(count * 4)` and zeroed at `0x511456`, so the read
lands 424 bytes past a 100-byte block and the town byte is whatever heap data
follows it - observed values include 40, 95, 104, 228, 242 and 255. Drawing the
list then reads past the name bank and treats what it finds as a string pointer:

- when the value aliases readable memory, the row draws a wrong town ("Edinburgh",
  because slot 40 is the first entry of the adjacent full town-name table) or a
  blank - this is why the bug looks intermittent;
- when it does not, the game dies on the spot, with no dump and no error dialog.

Only the list is affected: the letter body and header are formatted at creation
through a bounded town-name helper, so reading a letter is always safe.

## The fix

The unbounded lookup is detoured (7 bytes at `0x47d928`, original bytes verified
before patching): town bytes below 40 read the bank exactly as before, anything
else draws an empty string - the same blank town column the unpatched game shows
on the days it happens to survive the read. Letters whose garbage byte lands in
range (0..39) still display that town name, as they always did; there is no way
to tell those apart from genuine towns at draw time.

The buggy letter creation itself is left untouched: an out-of-range byte is only
ever consumed by the list draw this mod bounds, and the mod has to work on saves
that already contain such letters, where the byte is long since written.

## Fixing the data instead

The bug is one wrong byte in one data file, so it can also be fixed at the
source. `missions_addon/patrouille.p2m` in this folder is the vanilla script with
that byte corrected - file offset `0x27e` (byte 639), `0x83` -> `0x00`, and
nothing else:

```
command 37   f7 02 83 05 02 01 00 00 f3 06 00 00   letter town = variable 131
             f7 02 00 05 02 01 00 00 f3 06 00 00   letter town = variable 0
```

Variable 0 is the town the letter is about, and the script says so five times
over: command 36 (the route command) writes the next town of the patrol route
into it, rotating the previous three through variables 10, 11 and 12; the letter
body prints that variable twice ("The next destination ... is `%t[v0]` ... get
your ship, `%s[v8]`, to `%t[v0]`"); command 38 computes the mission's next
deadline from it; command 51 tests arrival with `town_of_ship(v8) == v0`; and
every other letter in the script already passes `00` as its town operand.
Command 37 is the only one that does not, and it is the only out-of-range letter
town variable in any of the game's 94 mission scripts.

To use it, copy the `missions_addon` folder into the game folder, next to
`Patrician3.exe`, so the file ends up at
`<game folder>\missions_addon\patrouille.p2m`.

The game reads a loose file in preference to the copy inside `p2arch0_eng.cpr`,
and no command changes length, so the script's offset table and any program
counter stored in a savegame stay valid.

What this does **not** do is repair letters that already exist. The town byte is
written into the 16-byte message when the letter is created and saved with the
game, so every "Patrol destination" letter already in a mailbox keeps its garbage
byte and still crashes the list. The data fix only makes new letters correct; the
DLL is what keeps an existing save openable. Installing both is the sensible
combination - and it also gives a way to see the data fix working: with the DLL
installed, old letters draw an empty town column while new ones draw the town
their body names.
