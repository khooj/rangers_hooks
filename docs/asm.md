# GetDynaMethod

Returns pointer to dynamic method from `dynamic table`.
For example: GetDynaMethod(TPersistent, 2) == ptr to TPersistent_GetNamePath

How it works:
- func gets address of vmt of the class (eax) and desired method index (si)
- it checks if class have dyntable, if not - searches it in parent
- in cycles up through hierarchy if any parent has dyntable
- if dyntable is found then it compares its indexes to be equal to desired method index
- if index not found - just return
- if index found - then calculates offset to method pointer and returns it in ESI

# _CallDynaInst

Gets pointer to vmt of the class (eax) and desired method index in ecx (basically delphi wrapper function to asm code in case of arguments), calls `GetDynaMethod` and jumps to returned method if any.

# _CallDynaClass

Same as `_CallDynaInst` but receives address of vmt of the class.

# _FindDynaInst

Gets pointer to vmt of the class (eax), method index? (edx), calls `GetDynaMethod` and returns if method found else returns? AbstractError.

# _FindDynaClass

Same as `_FindDynaInst` but receives address of vmt of the class.