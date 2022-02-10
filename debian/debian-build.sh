#/bin/sh

mkdir -p ./pkgroot/{etc/systemd/system,bin}

if (( $# > 0 ))
then
	if [ $1 == "--use-musl" ];
	then
		RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target=$(arch)-unknown-linux-musl --manifest-path=../Cargo.toml
		mv ../target/$(arch)-unknown-linux-musl/release/amd-gpu-fan-daemon ./pkgroot/bin
	fi
else
	RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target=$(arch)-unknown-linux-gnu --manifest-path=../Cargo.toml
	mv ../target/$(arch)-unknown-linux-gnu/release/amd-gpu-fan-daemon ./pkgroot/bin
fi

strip ./pkgroot/bin/amd-gpu-fan-daemon

cp ../amd-gpu-fan.service ./pkgroot/etc/systemd/system
cp -r ./DEBIAN ./pkgroot

pkgversion=$(cat ../Cargo.toml | grep version | grep -o '".*"' | sed 's/"//g' | sed 's/[^0-9.]*//g')
pkgname=$(cat ../Cargo.toml | grep name | grep -o '".*"' | sed 's/"//g')
pkgdescription=$(cat ../Cargo.toml | grep description | grep -o '".*"' | sed 's/"//g')
pkgarchitecture=$(dpkg-architecture -q DEB_HOST_ARCH)
pkgsize=$(du -s ./pkgroot | cut -f1)
pkgfile=$(echo "./"${pkgname}"_"${pkgarchitecture}"_V"${pkgversion}".deb")

echo "Installed-Size: "$pkgsize >> ./pkgroot/DEBIAN/control
echo "Description: "$pkgdescription >> ./pkgroot/DEBIAN/control
echo "Version: "$pkgversion >> ./pkgroot/DEBIAN/control
echo "Architecture: "$pkgarchitecture >> ./pkgroot/DEBIAN/control

dpkg-deb -b ./pkgroot ${pkgfile}
sha512sum ${pkgfile} >> ${pkgfile}.sha512

rm -rf ./pkgroot
