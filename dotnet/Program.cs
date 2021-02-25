using System;
using System.Runtime.InteropServices;
using System.Text;
using System.Collections.Generic;
using System.Linq;

public class FFI {
    [DllImport("librust_ffi_example")]
    private static extern void simple_hello();
    public static void callHelloInRust()
    {
        simple_hello();
    }

    [DllImport("librust_ffi_example")]
    private static extern void transfer_simple_variable(Int32 number, Boolean b);
    public static void transferSimpleVariable(Int32 number, Boolean b) {
        transfer_simple_variable(number, b);
    }

    [StructLayout(LayoutKind.Sequential)]
    public class rSimpleStruct {
        public Int32 inner;
    }

    [DllImport("librust_ffi_example")]
    private static extern rSimpleStruct transfer_simple_struct(rSimpleStruct simple_struct);
    public static rSimpleStruct transferSimpleStruct(rSimpleStruct simple_struct) {
        return transfer_simple_struct(simple_struct);
    }


    private static IntPtr _complicated_struct;
    [DllImport("librust_ffi_example")]
    private static extern void init_complicated_struct(out IntPtr complicated_struct, Int32 inner);
    public static void initComplicatedStruct(Int32 inner) {
        init_complicated_struct(out _complicated_struct, inner);
    }

    [DllImport("librust_ffi_example")]
    private static extern Int32 get_inner(IntPtr complicated_struct);
    public static Int32 getInnerOfComplicatedStruct() {
        return get_inner(_complicated_struct);
    }
}

namespace dotnet
{
    class Program
    {
        static void Main(string[] args) {
            FFI.callHelloInRust();
            FFI.transferSimpleVariable(1, true);

            FFI.rSimpleStruct input = new FFI.rSimpleStruct();
            input.inner = 2;
            FFI.rSimpleStruct output = FFI.transferSimpleStruct(input);
            Console.WriteLine(output.inner); /// Output is 4

            FFI.initComplicatedStruct(3);
            Console.WriteLine(FFI.getInnerOfComplicatedStruct()); /// Output is 3
        }
    }
}
