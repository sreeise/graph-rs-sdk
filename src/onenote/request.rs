use crate::client::Graph;
use crate::http::{GraphResponse, ResponseClient};
use crate::types::collection::Collection;
use crate::url::{FormatOrd, UrlOrdering};
use graph_rs_types::entitytypes::Notebook;
use graph_rs_types::entitytypes::OnenotePage;
use graph_rs_types::entitytypes::OnenoteSection;
use graph_rs_types::entitytypes::SectionGroup;
use reqwest::Method;
use std::marker::PhantomData;

fn ord_extend(mut url_ord: Vec<FormatOrd>) -> Vec<FormatOrd> {
    url_ord.push(FormatOrd::Insert(UrlOrdering::ItemPath("onenote".into())));
    url_ord
}

fn id_ord(root: &str, last: &str) -> Vec<FormatOrd> {
    ord_extend(vec![
        FormatOrd::Insert(UrlOrdering::RootOrItem(root.into())),
        FormatOrd::Insert(UrlOrdering::Last(last.into())),
    ])
}

client_struct!(OneNoteRequest);

impl<'a, I> OneNoteRequest<'a, I> {
    pub fn note_books(&self) -> OneNoteNoteBookRequest<'a, I> {
        OneNoteNoteBookRequest::new(self.client)
    }

    pub fn sections(&self) -> OneNoteSectionRequest<'a, I> {
        OneNoteSectionRequest::new(self.client)
    }

    pub fn section_group(&self) -> OneNoteSectionGroupRequest<'a, I> {
        OneNoteSectionGroupRequest::new(self.client)
    }

    pub fn pages(&self) -> OneNotePageRequest<'a, I> {
        OneNotePageRequest::new(self.client)
    }
}

client_struct!(OneNoteNoteBookRequest);

impl<'a, I> OneNoteNoteBookRequest<'a, I> {
    get!(
        list,
        Collection<Notebook>,
        ord_extend(vec![FormatOrd::Insert(UrlOrdering::RootOrItem(
            "notebooks".into()
        ))])
    );
    get!(
        list_sections,
        Collection<OnenoteSection>,
        id_ord("notebooks", "sections"),
        false
    );
    get!(
        get,
        Notebook,
        ord_extend(vec![FormatOrd::Insert(UrlOrdering::RootOrItem(
            "notebooks".into()
        ))]),
        false
    );
    post!(
        create,
        Notebook,
        ord_extend(vec![FormatOrd::Insert(UrlOrdering::RootOrItem(
            "notebooks".into()
        ))]),
        true,
        ()
    );
    post!(
        create_section,
        OnenoteSection,
        id_ord("notebooks", "sections"),
        false,
        ()
    );
    post!(
        copy,
        Notebook,
        id_ord("notebooks", "copyNotebook"),
        false,
        ()
    );

    pub fn recent_note_books(
        &self,
        include_personal_notebooks: bool,
    ) -> ResponseClient<'a, I, Notebook> {
        self.client
            .request()
            .insert(UrlOrdering::Last(format!(
                "onenote/notebooks/getRecentNotebooks(includePersonalNotebooks={})",
                include_personal_notebooks
            )))
            .set_method(Method::GET)
            .format_ord();
        ResponseClient::new(self.client)
    }
}

client_struct!(OneNoteSectionRequest);

impl<'a, I> OneNoteSectionRequest<'a, I> {
    get!(
        list,
        Collection<OnenoteSection>,
        ord_extend(vec![FormatOrd::Insert(UrlOrdering::Last(
            "sections".into()
        ))])
    );
    get!(
        list_pages,
        Collection<OnenotePage>,
        id_ord("sections", "pages"),
        false
    );
    get!(
        get,
        OnenoteSection,
        ord_extend(vec![FormatOrd::Insert(UrlOrdering::RootOrItem(
            "sections".into()
        ))]),
        false
    );
    post!(
        copy_to_notebook,
        GraphResponse<()>,
        id_ord("sections", "copyToNotebook"),
        false,
        ()
    );
    post!(
        copy_to_section_group,
        GraphResponse<()>,
        id_ord("sections", "copyToSectionGroup"),
        false,
        ()
    );
}

client_struct!(OneNoteSectionGroupRequest);

impl<'a, I> OneNoteSectionGroupRequest<'a, I> {
    get!(
        list,
        Collection<SectionGroup>,
        id_ord("sectionGroups", "sectionGroups"),
        false
    );
    get!(
        list_sections,
        Collection<OnenoteSection>,
        id_ord("sectionGroups", "sections"),
        false
    );
    get!(
        get,
        SectionGroup,
        ord_extend(vec![FormatOrd::Insert(UrlOrdering::RootOrItem(
            "sectionGroups".into()
        ))]),
        false
    );
    post!(
        create,
        SectionGroup,
        id_ord("sectionGroups", "sectionGroups"),
        false,
        ()
    );
    post!(
        create_section,
        OnenoteSection,
        id_ord("sectionGroups", "sections"),
        false,
        ()
    );
}

client_struct!(OneNotePageRequest);

impl<'a, I> OneNotePageRequest<'a, I> {
    get!(
        list,
        Collection<OnenotePage>,
        ord_extend(vec![FormatOrd::Insert(UrlOrdering::Last("pages".into()))])
    );
    get!(
        get,
        OnenotePage,
        ord_extend(vec![FormatOrd::Insert(UrlOrdering::RootOrItem(
            "pages".into()
        ))]),
        false
    );
    patch!(update, OnenotePage, id_ord("pages", "content"), false, ());
    post!(
        copy_to_section,
        GraphResponse<()>,
        id_ord("pages", "copyToSection"),
        false,
        ()
    );
    delete!(
        delete,
        GraphResponse<()>,
        ord_extend(vec![FormatOrd::Insert(UrlOrdering::RootOrItem(
            "pages".into()
        ))]),
        false
    );
}
