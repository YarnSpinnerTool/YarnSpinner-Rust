//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Tests/LanguageTests.cshttps://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Tests/LanguageTests.cs>

mod test_base;
use crate::test_base::*;

#[test]
#[ignore]
fn test_example_script() {
    todo!()
}

#[test]
#[ignore]
fn test_merging_nodes() {
    todo!()
}

#[test]
#[ignore]
fn test_end_of_notes_with_options_not_added() {
    todo!()
}

#[test]
fn test_node_headers() {
    todo!()
    /*
    let path = TestBase::test_data_path().join("Headers.yarn");
    let result = Compiler.Compile(CompilationJob.CreateFromFiles(path));

    result.Diagnostics.Should().BeEmpty();

    result.Program.Nodes.Count.Should().Be(6);

    foreach (let tag in new[] {"one", "two", "three"})
    {
        result.Program.Nodes["Tags"].Tags.Should().Contain(tag);
    }

    let headers = new Dictionary<string, List<(string, string)>>();
    headers.Add("EmptyTags", new List<(string, string)>{
        ("title","EmptyTags"),
        ("tags", null)
    });
    headers.Add("ArbitraryHeaderWithValue", new List<(string, string)>{
        ("title", "ArbitraryHeaderWithValue"),
        ("arbitraryheader", "some-arbitrary-text")
    });
    headers.Add("Tags", new List<(string, string)>{
        ("title", "Tags"),("tags",
                           "one two three")
    });
    headers.Add("SingleTagOnly", new List<(string, string)>{
        ("title", "SingleTagOnly")
    });
    headers.Add("Comments", new List<(string, string)>{
        ("title", "Comments"),
        ("tags", "one two three")
    });
    headers.Add("LotsOfHeaders", new List<(string, string)>{
        ("contains", "lots"),
        ("title", "LotsOfHeaders"),
        ("this", "node"),
        ("of", null),
        ("blank", null),
        ("others", "are"),
        ("headers", ""),
        ("some", "are"),
        ("not", "")
    });

    headers.Count.Should().Be(result.Program.Nodes.Count);
    foreach (let pair in headers)
    {
        result.Program.Nodes[pair.Key].Headers.Count.Should().Be(pair.Value.Count);

        // go through each item in the headers and ensure they are in the header list
        foreach (let header in result.Program.Nodes[pair.Key].Headers)
        {
            let match = pair.Value.Where(t => t.Item1.Equals(header.Key)).First();
            match.Item1.Should().Be(header.Key);

            if (match.Item2 == null)
            {
                header.Value.Should().BeNullOrEmpty();
            }
            else
            {
                match.Item2.Should().Be(header.Value);
            }
        }
    }

    // result.FileTags.Should().Contain("version:2");
    result.FileTags.Keys.Should().Contain(path);
    result.FileTags.Should().ContainSingle();
    result.FileTags[path].Should().ContainSingle();
    result.FileTags[path].Should().Contain("file_header");
    */
}

#[test]
#[ignore]
fn test_invalid_characters_in_node_title() {
    todo!()
}

#[test]
#[ignore]
fn test_number_plurals() {
    todo!()
}

#[test]
#[ignore]
fn test_compilation_should_not_be_culture_dependent() {
    todo!()
}

#[test]
#[ignore]
fn test_sources() {}
