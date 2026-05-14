# Maintainer: Rudra Saraswat <rs2009@ubuntu.com>
# Original Maintainer: Danct12 <danct12@disroot.org>
# Contributor: Bart Ribbers <bribbers@disroot.org>

pkgname=waydroid-blend-git
pkgver=1.4.0.r10.g74a5e54
pkgrel=1
pkgdesc="A container-based approach to boot a full Android system on a regular Linux system"
arch=('any')
provides=('waydroid')
conflicts=('waydroid' 'waydroid-git')
url='https://github.com/waydroid'
license=('GPL')
depends=('lxc' 'python-gbinder' 'python-gobject' 'nftables' 'dnsmasq' 'gtk3' 'dbus-python')
makedepends=('git')
optdepends=('python-pyclip: share clipboard with container')
source=("waydroid::git+https://github.com/waydroid/waydroid.git"
        gbinder.conf)

pkgver() {
  cd waydroid
  git describe --long --tags --abbrev=7 | sed 's/\([^-]*-g\)/r\1/;s/-/./g'
}

package() {
  make -C waydroid install DESTDIR="$pkgdir" USE_NFTABLES=1
  rm -f "${pkgdir}/usr/share/applications/Waydroid.desktop"
}

sha256sums=('SKIP'
            '87a21d401281735ea026d715ea79b36e01f9af084198de2761b32d5b58a343dd')
