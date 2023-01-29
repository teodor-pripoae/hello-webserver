package systems.toni.hello.webserver

import io.quarkus.test.junit.QuarkusTest
import io.restassured.RestAssured.given
import org.hamcrest.CoreMatchers.`is`
import org.junit.jupiter.api.Test

@QuarkusTest
class GreetingResourceTest {

    @Test
    fun testHelloEndpoint() {
        given().`when`().get("/hello").then().statusCode(200).body(`is`("hello, you've hit /hello"))
    }

    @Test
    fun testHelloWorldEndpoint() {
        given().`when`().get("/world").then().statusCode(200).body(`is`("hello, you've hit /world"))
    }
}
