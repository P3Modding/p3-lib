# mod-crash-reporter

Writes a crash report to `_crash_report.txt` in the game directory whenever the
game raises a fatal exception. Patrician 3 normally dies to desktop silently - no
minidump, no error dialog, nothing in DebugView - which makes crashes nearly
impossible to diagnose; with this mod loaded, every crash leaves a report behind.

Install: put `crash_reporter.dll` into the `mods` folder (requires the
modloader). On load it prints `crash_reporter: installed` to DebugView, and one
line per written report.

## How it works

A vectored exception handler registered at the front of the chain sees every
exception first-chance, before any of the game's own handlers run - so the report
is on disk even if something swallows the exception or kills the process
afterwards. Only fatal-severity codes are reported; C++ throws, debug prints and
breakpoints are routine dispatch traffic and are skipped. An unhandled-exception
filter additionally marks reports the process did not survive, when the game's
own filter does not preempt it.

## Reading a report

Each report contains:

- the exception code and faulting address, attributed as `module+offset`
  (`Patrician3.exe` loads at its preferred base `0x400000` with an empty
  relocation table, so game offsets translate 1:1 to disassembly addresses);
  for access violations, whether it was a read, write or execute, and of what
  address
- all registers
- a hex dump of the code around EIP, the faulting byte marked with `>`
- a hex dump of the memory around each register value that points into readable
  memory - the crash-time contents of whatever structures the faulting code was
  working on
- the EBP frame chain, and a scan of the stack for values pointing into module
  code (return-address candidates, which recover the call history through
  optimized frames that omit frame pointers)

Reports are appended, numbered per game launch. A report the process survived was
a handled exception - the last report in the file is the crash.

## Blind spot

Heap-corruption kills go through fail-fast (`int 29`) on modern Windows, which
never enters exception dispatch: a crash that leaves no report points at the
heap. Enabling PageHeap (`gflags /p /enable Patrician3.exe /full`) makes the
corruption fault at the corrupting instruction instead, which this reporter
catches.
