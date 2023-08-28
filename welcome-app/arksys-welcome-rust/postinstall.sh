#!/bin/sh

mkdir -p "${DESTDIR}/${MESON_INSTALL_PREFIX}/share/arksys-welcome/"
cp -r "${MESON_SOURCE_ROOT}/src/scripts" "${DESTDIR}/${MESON_INSTALL_PREFIX}/share/arksys-welcome/"
cp -r "${MESON_SOURCE_ROOT}/data" "${DESTDIR}/${MESON_INSTALL_PREFIX}/share/arksys-welcome/"
cp -r "${MESON_SOURCE_ROOT}/ui" "${DESTDIR}/${MESON_INSTALL_PREFIX}/share/arksys-welcome/"

cd "${MESON_SOURCE_ROOT}/po"
for lang in $(ls *.po); do
    lang=${lang::-3}
    mkdir -p ${DESTDIR}/usr/share/locale/${lang//_/-}/LC_MESSAGES
    msgfmt -c -o ${DESTDIR}/usr/share/locale/${lang//_/-}/LC_MESSAGES/cachyos-hello.mo $lang.po
done
