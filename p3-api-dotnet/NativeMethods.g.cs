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
        const string __DllName = "p3-api-dotnet";

        /// <summary>Open Patrician3.exe for Reading.</summary>
        [DllImport(__DllName, EntryPoint = "new_api", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern OpenProcessP3AccessApi* new_api(uint pid);

        /// <summary>Reads a ship from memory.  # Safety  `api` must point toward a valid OpenProcessP3AccessApi.</summary>
        [DllImport(__DllName, EntryPoint = "read_ship", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern DotnetShip* read_ship(OpenProcessP3AccessApi* api, ushort ship_id);


    }



}
    