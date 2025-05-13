# Maintainer: Alex Lost <astro.tofuu@gmail.com>
pkgname=yts-cli
pkgver=0.0.2
pkgrel=1
pkgdesc="A tool to list movies from YTS and generate magnet links"
arch=('x86_64' 'aarch64')
url="https://github.com/Ka-derka/yts-cli"
license=('MIT')
depends=('curl')                     # runtime dependencies
makedepends=('rust' 'cargo' 'base-devel')  # build-time dependencies
source=("$pkgname-$pkgver.tar.gz::https://github.com/Ka-derka/yts-cli/archive/refs/tags/v${pkgver}.tar.gz")
sha512sums=('SKIP')                   # replace with real sha512sum for release tarball

build() {
  cd "$srcdir/$pkgname-$pkgver"
  cargo build --release
}

package() {
  cd "$srcdir/$pkgname-$pkgver"
  install -Dm755 "target/release/yts-cli" "$pkgdir/usr/bin/yts-cli"
  install -Dm644 README.md            "$pkgdir/usr/share/licenses/$pkgname/README.md"
  install -Dm644 LICENSE              "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
