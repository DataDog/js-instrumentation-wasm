// No strings in this file should appear in the dictionary.
let foo: 'unwanted_var_type_1' | "unwanted_var_type_2";

function bar(
  x: "unwanted_arg_type_1",
  y: 'unwanted_arg_type_2'
): 'unwanted_return_value_type_3' | undefined {
  return undefined;
}

type ObjectType = {
  'unwanted_object_key_type_1': 'unwanted_object_value_type_1'
};

interface InterfaceType {
  'unwanted_interface_key_type_1': 'unwanted_interface_value_type_1'
};

declare let baz: 'unwanted_declared_var_type_1';
