Outfile "WaxEncInstaller.exe"
Name "WaxEnc Installer"

InstallDir "$PROGRAMFILES\WaxEnc"

Section "Install"
    SetOutPath "$INSTDIR"

    File "waxenc.exe"
    File "waxe-file.ico"
    File "waxd-file.ico"
    WriteUninstaller "$INSTDIR\uninstall.exe"

    WriteRegStr HKCR ".waxd" "" "waxdfile"
    WriteRegStr HKCR ".waxe" "" "waxefile"
    WriteRegStr HKCR "waxdfile" "" "Wax Decrypted file"
    WriteRegStr HKCR "waxefile" "" "Wax Encrypted file"
    WriteRegStr HKCR "*\shell\Encrypt with Wax" "Icon" "$INSTDIR\waxe-file.ico"
    WriteRegStr HKCR "*\shell\Decrypt with Wax" "Icon" "$INSTDIR\waxd-file.ico"
    WriteRegStr HKCR "waxefile\shell\Decrypt with Wax" "Icon" "$INSTDIR\waxd-file.ico"
    WriteRegStr HKCR "waxefile\shell\Decrypt with Wax\command" "" "$INSTDIR\waxenc.exe dec %1"
    WriteRegStr HKCR "*\shell\Encrypt with Wax\command" "" "$INSTDIR\waxenc.exe enc %1"
    WriteRegStr HKCR "*\shell\Decrypt with Wax\command" "" "$INSTDIR\waxenc.exe dec %1"

SectionEnd

Section "Uninstall"
    Delete "$INSTDIR\waxenc.exe"
    Delete "$INSTDIR\uninstall.exe"
    Delete "$INSTDIR\waxe-file.ico"
    Delete "$INSTDIR\waxd-file.ico"
    RMDir "$INSTDIR"

    DeleteRegKey HKCR "waxefile\shell\Decrypt with Wax\command"
    DeleteRegKey HKCR "waxefile\shell\Decrypt with Wax"
    DeleteRegKey HKCR "waxefile"
    DeleteRegKey HKCR "waxdfile"
    DeleteRegKey HKCR "*\shell\Encrypt with Wax"
    DeleteRegKey HKCR "*\shell\Decrypt with Wax"
    DeleteRegKey HKCR ".waxe"
    DeleteRegKey HKCR ".waxd"

SectionEnd
