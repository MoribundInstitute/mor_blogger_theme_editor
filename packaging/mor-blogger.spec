# Binary repackage of the dx-bundled deb: extract payload/ with `dpkg-deb -x <deb> payload`, build with rpmbuild --define "payload_dir ..." --define "_sourcedir ..." (needs LICENSE in sourcedir).
%define debug_package %{nil}

Name:    mor-blogger-dioxus-ui
Version: 0.1.0
Release: 1
Summary: A visual Blogger theme editor
License: MIT
URL:     https://github.com/MoribundInstitute/mor_blogger_theme_editor

%description
MorBlogger Theme Editor is a Dioxus desktop app for visually building,
previewing, and exporting Blogger themes.

%install
cp -a %{payload_dir}/. %{buildroot}
install -Dm644 %{_sourcedir}/LICENSE %{buildroot}/usr/share/licenses/%{name}/LICENSE

%files
/usr/bin/mor_blogger_dioxus_ui
/usr/lib/MorBloggerDioxusUi
/usr/share/applications/mor_blogger_dioxus_ui.desktop
/usr/share/icons/hicolor/512x512/apps/mor_blogger_dioxus_ui.png
/usr/share/licenses/%{name}/LICENSE
