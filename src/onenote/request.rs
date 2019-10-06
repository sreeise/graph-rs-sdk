use crate::client::Graph;
use crate::http::{GraphResponse, IntoResponse};
use crate::types::collection::Collection;
use graph_rs_types::entitytypes::{Notebook, OnenotePage, OnenoteSection, SectionGroup};
use handlebars::*;
use reqwest::Method;
use std::marker::PhantomData;

register_client!(
    OnenoteRequest,
    notebook => "onenote/notebooks",
    section => "onenote/sections",
    section_group => "onenote/sectionGroups",
    pages => "onenote/pages",
);

impl<'a, I> OnenoteRequest<'a, I> {
    pub fn notebooks(&self) -> OnenoteNotebookRequest<'a, I> {
        OnenoteNotebookRequest::new(self.client)
    }

    pub fn sections(&self) -> OnenoteSectionRequest<'a, I> {
        OnenoteSectionRequest::new(self.client)
    }

    pub fn section_group(&self) -> OnenoteSectionGroupRequest<'a, I> {
        OnenoteSectionGroupRequest::new(self.client)
    }

    pub fn pages(&self) -> OnenotePageRequest<'a, I> {
        OnenotePageRequest::new(self.client)
    }
}

register_client!(OnenoteNotebookRequest,);

impl<'a, I> OnenoteNotebookRequest<'a, I> {
    get!( list, Collection<Notebook> => "{{notebook}}" );
    get!( | list_sections, Collection<OnenoteSection> => "{{notebook}}/{{id}}/sections" );
    get!( | get, Notebook => "{{notebook}}/{{id}}" );
    post!( [ create, Notebook => "{{notebook}}" ]);
    post!( [ | copy, OnenoteSection => "{{notebook}}/{{id}}/copyNotebook" ] );
    post!( [ | create_section, OnenoteSection => "{{notebook}}/{{id}}/sections" ] );

    pub fn recent(&self, include_personal_notebooks: bool) -> IntoResponse<'a, I, Notebook> {
        let s = format!(
            "onenote/notebooks/getRecentNotebooks(includePersonalNotebooks={})",
            include_personal_notebooks
        );
        let mut vec: Vec<&str> = s.split('/').collect();
        vec.retain(|s| !s.is_empty());
        self.client
            .builder()
            .set_method(Method::GET)
            .as_mut()
            .extend_path(&vec);
        IntoResponse::new(self.client)
    }
}

register_client!(OnenoteSectionRequest,);

impl<'a, I> OnenoteSectionRequest<'a, I> {
    get!( list, Collection<OnenoteSection> => "{{section}}" );
    get!( | list_pages, Collection<OnenotePage> => "{{section}}/{{id}}/pages" );
    get!( | get, OnenoteSection => "{{section}}/{{id}}" );
    post!( [ | copy_to_notebook, GraphResponse<()> => "{{section}}/{{id}}/copyToNotebook" ] );
    post!( [ | copy_to_section_group, GraphResponse<()> => "{{section}}/{{id}}/copyToSectionGroup" ] );
}

register_client!(OnenoteSectionGroupRequest,);

impl<'a, I> OnenoteSectionGroupRequest<'a, I> {
    get!( list, Collection<SectionGroup> => "{{section_group}}" );
    get!( | list_sections, Collection<OnenoteSection> => "{{section_group}}/{{id}}/sections" );
    get!( | get, SectionGroup => "{{section_group}}/{{id}}" );
    post!( [ | create, SectionGroup => "{{section_group}}/{{id}}/sectionGroups" ] );
    post!( [ | create_section, SectionGroup => "{{section_group}}/{{id}}/sections" ] );
}

register_client!(OnenotePageRequest,);

impl<'a, I> OnenotePageRequest<'a, I> {
    get!( list, Collection<OnenotePage> => "{{pages}}" );
    get!( | get, OnenotePage => "{{pages}}/{{id}}" );
    patch!( [ | update, OnenotePage => "{{pages}}/{{id}}/content" ] );
    post!( [ | copy_to_section, GraphResponse<()> => "{{pages}}/{{id}}/copyToSection" ] );
    delete!( | delete, GraphResponse<()> => "{{pages}}/{{id}}" );
}
