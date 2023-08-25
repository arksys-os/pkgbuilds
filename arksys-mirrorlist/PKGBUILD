# Maintainer: DAvid7ce
pkgname=arksys-mirrorlist
pkgver=0.1.3
pkgrel=3
pkgdesc="ArkSyS Configuration Tool"
arch=('any')
license=('GPL3')
makedepends=('git')
depends=()
conflicts=()
provides=("${pkgname}")
options=(!strip !emptydirs)
source=(${pkgname}::"git+https://github.com/arksys-os/${pkgname}")
sha256sums=('SKIP')
package() {
	install -dm755 ${pkgdir}
	cp -r ${srcdir}/${pkgname}/* ${pkgdir}
	rm "${pkgdir}${_destname1}/PKGBUILD"
}
