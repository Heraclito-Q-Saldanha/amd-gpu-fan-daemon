#/bin/sh

cd "$(dirname "${BASH_SOURCE[0]}")"

pkgversion=$(cat ../Cargo.toml | grep version | grep -o '".*"' | sed 's/"//g' | sed 's/[^0-9.]*//g')
pkgname=$(cat ../Cargo.toml | grep name | grep -o '".*"' | sed 's/"//g')
pkgdescription=$(cat ../Cargo.toml | grep description | grep -o '".*"' | sed 's/"//g')
pkgarchitecture=$(dpkg-architecture -q DEB_HOST_ARCH)
pkgsize=0
pkgfile=$(echo "./"${pkgname}"_"${pkgarchitecture}"_V"${pkgversion}".deb")


compile(){
	RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target=$(arch)-unknown-linux-gnu --manifest-path=../Cargo.toml
	mv ../target/$(arch)-unknown-linux-gnu/release/amd-gpu-fan-daemon ./pkgroot/bin
	strip ./pkgroot/bin/amd-gpu-fan-daemon
	pkgsize=$(du -s ./pkgroot | cut -f1)
}

compile_musl(){
	RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target=$(arch)-unknown-linux-musl --manifest-path=../Cargo.toml
	mv ../target/$(arch)-unknown-linux-musl/release/amd-gpu-fan-daemon ./pkgroot/bin
	strip ./pkgroot/bin/amd-gpu-fan-daemon
	pkgsize=$(du -s ./pkgroot | cut -f1)
}

config_files(){
	mkdir -p ./pkgroot/{etc/systemd/system,bin}
	cp ../amd-gpu-fan.service ./pkgroot/etc/systemd/system
	cp -r ./DEBIAN ./pkgroot
	chmod 0755 -R ./pkgroot
}

config_control(){
	echo "Installed-Size: "$pkgsize >> ./pkgroot/DEBIAN/control
	echo "Description: "$pkgdescription >> ./pkgroot/DEBIAN/control
	echo "Version: "$pkgversion >> ./pkgroot/DEBIAN/control
	echo "Architecture: "$pkgarchitecture >> ./pkgroot/DEBIAN/control
}

pack(){
	dpkg-deb -b ./pkgroot ${pkgfile}
	sha512sum ${pkgfile} > ${pkgfile}.sha512
}

if (( $# > 0 ))
then
	if [ $1 == "--use-musl" ];
	then
		config_files
		compile_musl
	else
		echo "sintax error "${1}
		exit
	fi
else
	config_files
	compile
fi
config_control
pack
rm -rf ./pkgroot


