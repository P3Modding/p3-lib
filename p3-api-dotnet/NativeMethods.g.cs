// <auto-generated>
// This code is generated by csbindgen.
// DON'T CHANGE THIS DIRECTLY.
// </auto-generated>
#pragma warning disable CS8500
#pragma warning disable CS8981
using System;
using System.Runtime.InteropServices;

namespace CsBindgen
{
    internal static unsafe partial class NativeMethods
    {
        const string __DllName = "p3_api_dotnet";

        /// <summary>Open Patrician3.exe for Reading.</summary>
        [DllImport(__DllName, EntryPoint = "new_api", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern DotnetOpenProcessP3AccessApi* new_api(uint pid);

        /// <summary>Reads a ship from memory.  # Safety  `api` must point toward a valid DotnetOpenProcessP3AccessApi.</summary>
        [DllImport(__DllName, EntryPoint = "read_ship", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern DotnetShip* read_ship(DotnetOpenProcessP3AccessApi* api, ushort ship_id);


    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct DotnetOpenProcessP3AccessApi
    {
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct DotnetShip
    {
        public byte merchant_id;
        public int max_health;
        public int current_health;
        public int x;
        public int y;
        public byte current_town_id;
    }



}
    