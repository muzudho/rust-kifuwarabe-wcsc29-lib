<#
 # ドキュメント作成。
 # $pro - product.
 # $ori - original.
 # $dep - deployment. Trailing NOT slash.
 #>
function Copy-Doc($pro, $ori, $dep) {
    # Recycle.
    if (Test-Path $dep) {
        Remove-Item $dep -Recurse
    }

    $excludeArray = @("*.md", "*.pu")
    Copy-Item $ori $dep -Recurse -Exclude $excludeArray
}
