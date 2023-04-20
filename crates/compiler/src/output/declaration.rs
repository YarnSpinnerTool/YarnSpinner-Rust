//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/Declaration.cs>

/*
   [Serializable]
   public class Declaration
   {
       /// <summary>
       /// Gets the name of this Declaration.
       /// </summary>
       public string Name { get; internal set; }

       /// <summary>
       /// Creates a new instance of the <see cref="Declaration"/> class,
       /// using the given name, types and default value.
       /// </summary>
       /// <param name="name">The name of the new declaration.</param>
       /// <param name="types">The types of the declaration.</param>
       /// <param name="defaultValue">The default value of the
       /// declaration. This must be a string, a number (integer or
       /// floating-point), or boolean value.</param>
       /// <param name="description">The description of the new
       /// declaration.</param>
       /// <returns>A new instance of the <see cref="Declaration"/>
       /// class.</returns>
       public static Declaration CreateVariable(string name, Yarn.IType types, IConvertible defaultValue, string description = null)
       {
           if (types is null)
           {
               throw new ArgumentNullException(nameof(types));
           }

           if (string.IsNullOrEmpty(name))
           {
               throw new ArgumentException($"'{nameof(name)}' cannot be null or empty.", nameof(name));
           }

           if (defaultValue is null)
           {
               throw new ArgumentNullException(nameof(defaultValue));
           }

           // We're all good to create the new declaration.
           return new Declaration
           {
               Name = name,
               DefaultValue = defaultValue,
               Type = types,
               Description = description,
           };
       }

       /// <summary>
       /// Gets the default value of this <see cref="Declaration"/>, if no
       /// value has been specified in code or is available from a <see
       /// cref="Dialogue"/>'s <see cref="IVariableStorage"/>.
       /// </summary>
       public IConvertible DefaultValue { get; internal set; }

       /// <summary>
       /// Gets a string describing the purpose of this <see
       /// cref="Declaration"/>.
       /// </summary>
       public string Description { get; internal set; }

       /// <summary>
       /// Gets the name of the file in which this Declaration was found.
       /// </summary>
       /// <remarks>
       /// If this <see cref="Declaration"/> was not found in a Yarn
       /// source file, this will be <see cref="ExternalDeclaration"/>.
       /// </remarks>
       public string SourceFileName { get; internal set; }

       /// <summary>
       /// Gets the name of the node in which this Declaration was found.
       /// </summary>
       /// <remarks>
       /// If this <see cref="Declaration"/> was not found in a Yarn
       /// source file, this will be <see langword="null"/>.
       /// </remarks>
       public string SourceNodeName { get; internal set; }

       /// <summary>
       /// Gets the line number at which this Declaration was found in the
       /// source file.
       /// </summary>
       /// <remarks>
       /// If this <see cref="Declaration"/> was not found in a Yarn
       /// source file, this will be -1.
       /// </remarks>
       public int SourceFileLine => this.Range.Start.Line;

       /// <summary>
       /// Gets a value indicating whether get or sets a value indicating
       /// whether this Declaration was implicitly inferred from usage.
       /// </summary>
       /// <value>If <see langword="true"/>, this Declaration was implicitly
       /// inferred from usage. If <see langword="false"/>, this Declaration
       /// appears in the source code.</value>
       public bool IsImplicit { get; internal set; }

       /// <summary>
       /// Gets the types of the variable, as represented by an object that
       /// implements <see cref="IType"/>.
       /// </summary>
       public Yarn.IType Type { get; internal set; }

       /// <summary>
       /// The string used for <see cref="SourceFileName"/> if the
       /// Declaration was found outside of a Yarn source file.
       /// </summary>
       public const string ExternalDeclaration = "(External)";

       /// <summary>
       /// Gets the range of text at which this declaration occurs.
       /// </summary>
       /// <remarks>
       /// This range refers to the declaration of the symbol itself, and not
       /// any syntax surrounding it. For example, the declaration
       /// <c>&lt;&lt;declare $x = 1&gt;&gt;</c> would have a Range referring
       /// to the <c>$x</c> symbol.
       /// </remarks>
       public Range Range { get; internal set; } = new Range();
*/

use rusty_yarn_spinner_core::prelude::{Type, Value};
use std::ops::RangeInclusive;

/// Information about a declaration. Stored inside a declaration table,
/// which is produced from the Compiler.
///
/// You do not create instances of this class yourself. They are
/// generated by the [`Compiler`].
#[derive(Debug, Clone)]
pub struct Declaration {
    /// The name of this declaration.
    pub name: String,

    /// The default value of this declaration, if no value has been
    /// specified in code or is available from a [`Dialogue`]'s
    /// [`IVariableStorage`].
    pub default_value: Value,

    /// A string describing the purpose of this declaration.
    pub description: String,

    /// The name of the file in which this declaration was found.
    ///
    /// If this declaration was not found in a Yarn source file, this
    /// will be [`DeclarationSource::External`].
    pub source_file_name: DeclarationSource,

    /// The name of the node in which this declaration was found.
    ///
    /// If this declaration was not found in a Yarn source file, this
    /// will be [`None`].
    pub source_node_name: Option<String>,

    /// The line number at which this declaration was found in the
    /// source file.
    ///
    /// If this declaration was not found in a Yarn source file, this
    /// will be [`None`].
    pub source_file_line: Option<usize>,

    /// A value indicating whether this declaration was implicitly
    /// inferred from usage.
    ///
    /// If `true`, this declaration was implicitly inferred from usage.
    /// If `false`, this declaration appears in the source code.
    pub is_implicit: bool,

    /// The types of the variable, as represented by an object found
    /// in a variant of [`DeclarationType`].
    pub r#type: Type, // TODO

    /// The range of text at which this declaration occurs.
    ///
    /// This range refers to the declaration of the symbol itself, and
    /// not any syntax surrounding it. For example, the declaration
    /// `<<declare $x = 1>>` would have a range referring to the `$x`
    /// symbol.
    pub range: RangeInclusive<Position>,
}

#[derive(Debug, Clone)]
pub enum DeclarationType {}

#[derive(Debug, Clone)]
pub enum DeclarationSource {
    External,
    File(String),
}

/*
/// <summary>
/// Represents a position in a multi-line string.
/// </summary>
[System.Serializable]
public class Position
{
    /// <summary>
    /// Gets or sets the zero-indexed line of this position.
    /// </summary>
    public int Line { get; set; } = -1;

    /// <summary>
    /// Gets or sets the zero-indexed character number of this position.
    /// </summary>
    public int Character { get; set; } = -1;

    /// <inheritdoc/>
    public override bool Equals(object obj)
    {
        return obj is Position position &&
               this.Line == position.Line &&
               this.Character == position.Character;
    }

    /// <inheritdoc/>
    public override int GetHashCode()
    {
        int hashCode = 1927683087;
        hashCode = (hashCode * -1521134295) + this.Line.GetHashCode();
        hashCode = (hashCode * -1521134295) + this.Character.GetHashCode();
        return hashCode;
    }
}
*/

/// Represents a position in a multi-line string.
#[derive(Debug, Clone)]
pub struct Position {
    /// The zero-indexed line of this position.
    pub line: usize,

    /// The zero-indexed character number of this position.
    pub character: usize,
}
